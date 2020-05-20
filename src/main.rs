use {
    async_std::{
        net::{TcpListener, TcpStream},
        stream::StreamExt,
        task,
    },
    async_tls::{TlsAcceptor, server::TlsStream},
    lazy_static::lazy_static,
    rustls::internal::pemfile::{certs, pkcs8_private_keys},
    std::{
        error::Error,
        fs::{File, read},
        io::BufReader,
        path::{Path, PathBuf},
        sync::Arc,
    },
    url::Url,
};

pub type Result<T=()> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

lazy_static! {
    static ref ARGS: Args = args().expect("usage: agate <addr:port> <dir> <cert> <key>");
    static ref ACCEPTOR: TlsAcceptor = acceptor().unwrap();
}

struct Args {
    sock_addr: String,
    content_dir: String,
    cert_file: String,
    key_file: String,
}

fn main() -> Result {
    task::block_on(async {
        let listener = TcpListener::bind(&ARGS.sock_addr).await?;
        let mut incoming = listener.incoming();
        while let Some(Ok(stream)) = incoming.next().await {
            task::spawn(async {
                if let Err(e) = connection(stream).await {
                    eprintln!("Error: {:?}", e);
                }
            });
        }
        Ok(())
    })
}

fn args() -> Option<Args> {
    let mut args = std::env::args().skip(1);
    Some(Args {
        sock_addr: args.next()?,
        content_dir: args.next()?,
        cert_file: args.next()?,
        key_file: args.next()?,
    })
}

fn acceptor() -> Result<TlsAcceptor> {
    let cert_file = File::open(&ARGS.cert_file)?;
    let key_file = File::open(&ARGS.key_file)?;

    let certs = certs(&mut BufReader::new(cert_file)).or(Err("bad cert"))?;
    let mut keys = pkcs8_private_keys(&mut BufReader::new(key_file)).or(Err("bad key"))?;
    let mut config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
    config.set_single_cert(certs, keys.remove(0))?;
    Ok(TlsAcceptor::from(Arc::new(config)))
}

async fn connection(stream: TcpStream) -> Result {
    use async_std::io::prelude::*;
    let mut stream = ACCEPTOR.accept(stream).await?;
    let url = match parse_request(&mut stream).await {
        Ok(url) => url,
        Err(e) => {
            stream.write_all(b"50 Invalid request.\r\n").await?;
            return Err(e)
        }
    };
    match get(&url) {
        Ok(response) => {
            stream.write_all(b"20 text/gemini\r\n").await?;
            stream.write_all(&response).await?;
        }
        Err(e) => {
            stream.write_all(b"40 Not found, sorry.\r\n").await?;
            return Err(e)
        }
    }
    Ok(())
}

async fn parse_request(stream: &mut TlsStream<TcpStream>) -> Result<Url> {
    use async_std::io::prelude::*;
    let mut stream = async_std::io::BufReader::new(stream);
    let mut request = String::new();
    stream.read_line(&mut request).await?;
    let url = Url::parse(request.trim())?;
    Ok(url)
}

fn get(url: &Url) -> Result<Vec<u8>> {
    let mut path = PathBuf::from(&ARGS.content_dir);
    path.extend(url.path_segments().unwrap());
    if !path.starts_with(&ARGS.content_dir) {
        Err("invalid path")?
    }
    eprintln!("Got request for {:?}", path);
    let response = if path.is_dir() {
        list(&path)?
    } else {
        read(&path)?
    };
    Ok(response)
}

fn list(path: &Path) -> Result<Vec<u8>> {
    use std::io::Write;
    let mut result = vec![];
    for entry in path.read_dir()? {
        let entry = entry?;
        let file_name = entry.file_name().into_string()
            .or(Err("non-Unicode path"))?;
        let path = entry.path();
        let url = path.strip_prefix(&ARGS.content_dir)?.to_str()
            .ok_or("non-Unicode path")?;
        // TODO: Escape whitespace
        writeln!(&mut result, "=> {} {}", url, file_name)?;
    }
    Ok(result)
}

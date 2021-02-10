# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
Agate now has an explicit code of conduct and contributing guidelines.
Thank you to @gegeweb for contributing to this release.

### Added
* You can now supply multiple `--hostname`s to enable basic vhosts (#28).
* Disabling support for TLSv1.2 can now be done using the `--only-tls13` flag, but this is *NOT RECOMMENDED* (#12).
* The tools now also contain a startup script for FreeBSD (#13).

### Changed
* The changelog is now also kept in this file in addition to the GitHub releases.

### Fixed
* The Syntax for the IPv6 address in the README has been corrected.

## [2.4.1] - 2020-02-08
### Fixed
* Re-enabled multiple occurrences of `--addr`. This was accidentally disabled by a merge.

## [2.4.0]+podman.build - 2020-02-06
This is the same as [2.4.0], only the build process has been changed so it should accommodate a wider range of architectures and devices.

## [2.4.0] - 2020-02-06
Since there is a new maintainer (@Johann150), the range in pre-compiled binaries has changed a bit.

### Added
* Added some installation tools for Debian.
* Added a sidecar file for specifying languages, MIME media types or complete headers on a per file basis.

### Changed
* Improved logging output. Agate now also respects the `RUST_LOG` environment variable, so you can configure the log level.

## [2.3.0] - 2020-01-17
Thanks to @Johann150.

### Changed
* Combine address and port back into a single command-line argument (#21).

## [2.2.0] - 2020-01-16
Thank you to @Johann150 for contributing to this release.

### Changed
* Split address and port into separate command-line parameters.

### Fixed
* Listen on both IPv6 and IPv4 interfaces by default.
* fix the logic for detecting hidden files (#20).
* Fix redirects of URLs with query strings (#19).

## [2.1.3] - 2020-01-02
### Changed
* Switch to the Tokio async run time.

### Fixed
* Send TLS close-notify message when closing a connection.
* Require absolute URLs in requests.

## [2.1.2] - 2020-01-01
### Fixed
* More complete percent-encoding of special characters in filenames.
* Minor improvements to error logging.
* Internal code cleanup.

## [2.1.1] - 2020-12-31
### Changed
* List directory content in alphabetical order.

### Fixed
* Handle percent-escaped paths in URLs.
* Percent-escape white space characters in directory listings.

## [2.1.0] - 2020-12-29
* Enabled GitHub Discussions. If you are using Agate, please feel free to leave a comment to let us know about it!
Thank you to @Johann150 to contributing to this release.

### Added
* Optional directory listings (#9).

### Fixed
* Updated dependencies.

## [2.0.0] - 2020-12-23
### Added
* New `--language` option to add a language tag to the MIME type for text/gemini responses (#6).

### Changed
* New format for command-line options. See the documentation or run `agate --help` for details.
* Logging is enabled by default. Use the `--silent` flag to disable it.
* Pre-compiled binaries are built with the [`cross`](https://github.com/rust-embedded/cross) tool, for better compatibility with older Linux systems.

## [1.3.2] - 2020-12-09
This release is functionally identical to Agate 1.3.1, and users of that version do not need to update.

### Fixed
* Update to async-tls 0.11 because the previous version was [yanked](https://github.com/async-rs/async-tls/issues/42).

## [1.3.1] - 2020-12-08
Thanks @dcreager for contributing this fix.

### Fixed
* Updated dependencies to fix `cargo install` (#7).

## [1.3.0] - 2020-11-20
Thank you @Johann150 and @tronje for contributing to this release!

### Fixed
* verify hostname and port in request URL (#4).
* improved logging (#3).
* Don't redirect to "/" when the path is empty (#5).
* Update dependencies.

## [1.2.2] - 2020-09-21
### Changed
* Switch from `tree_magic` to `mime_guess` for simpler MIME type guessing.
* Built both x86_64 and ARM binaries. These binaries are built for Linux operating systems with glibc 2.28 or later, such as Debian 10 ("buster") or newer, Ubuntu 18.10 or newer, and Raspberry Pi OS 2019-06-20 or newer.

### Fixed
* Update dependencies.
* Minor internal code cleanup.

## [1.2.1] - 2020-06-20
### Fixed
* Reduce memory usage when serving large files.
* Update dependencies.

## [1.2.0] - 2020-06-10
### Changed
* text/gemini filename extension from `.gemini` to `.gmi`.

### Fixed
* Handling for requests that exceed 1KB.
* Reduce memory allocations and speed up request parsing.
* Update dependencies.

## [1.1.0] - 2020-05-22
### Added
* Auto-detect MIME types.

## [1.0.1] - 2020-05-21
### Added
* Send more accurate error codes for unsupported requests.
* Do more validation of request URLs.

## [1.0.0] - 2020-05-21

[Unreleased]: https://github.com/mbrubeck/agate/compare/v0.11.0...HEAD
[2.4.1]: https://github.com/mbrubeck/agate/compare/v2.4.0...v2.4.1
[2.4.0]: https://github.com/mbrubeck/agate/compare/v2.3.0...v2.4.0
[2.3.0]: https://github.com/mbrubeck/agate/compare/v2.2.0...v2.3.0
[2.2.0]: https://github.com/mbrubeck/agate/compare/v2.1.3...v2.2.0
[2.1.3]: https://github.com/mbrubeck/agate/compare/v2.1.2...v2.1.3
[2.1.2]: https://github.com/mbrubeck/agate/compare/v2.1.1...v2.1.2
[2.1.1]: https://github.com/mbrubeck/agate/compare/v2.1.0...v2.1.1
[2.1.0]: https://github.com/mbrubeck/agate/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/mbrubeck/agate/compare/v1.3.2...v2.0.0
[1.3.2]: https://github.com/mbrubeck/agate/compare/v1.3.1...v1.3.2
[1.3.1]: https://github.com/mbrubeck/agate/compare/v1.3.0...v1.3.1
[1.3.0]: https://github.com/mbrubeck/agate/compare/v1.2.2...v1.3.0
[1.2.2]: https://github.com/mbrubeck/agate/compare/v1.2.1...v1.2.2
[1.2.1]: https://github.com/mbrubeck/agate/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/mbrubeck/agate/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/mbrubeck/agate/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/mbrubeck/agate/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/mbrubeck/agate/releases/tag/v1.0.0
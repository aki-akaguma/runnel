# Changelog: runnel

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.3.19] (2024-06-19)
### Changed
* I changed a conditional compilation from `rust_version` to `rustversion`.

### Fixed
* clippy: dead_code

## [0.3.18] (2024-06-09)
### Changed
* rename: `config` to `config.toml`
* update crates: criterion(0.5)
* test support 1.60.0 on github workflows
* build support 1.60.0 on github workflows

## [0.3.17] (2023-02-12)
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`

### Changed
* refactored `Makefile`

### Removed
* `COPYING`

### Fixed
* `LICENSE-APACHE`, `LICENSE-MIT`
* clippy: `box_default`

## [0.3.16] (2023-01-28)
### Added
* `.github/workflows/test.yml`
* test status badges into `README.tpl`

### Fixed
* Makefile: rustc version `1.66.0` to `1.66.1`
* clippy: `box_default`
* `LICENSE` files
* bug: `test_size_of()` on macos and windows

## [0.3.15] (2023-01-10)
### Added
* version difference link into `CHANGELOG.md`
* badges into README.tpl

### Changed
* move benches into xbench

## [0.3.14] (2023-01-06)
### Added
* rust-version = "1.57.0" into Cargo.toml
* `all-test-version` target into Makefile

### Removed
* cfg `has_fat_stdout` from tests
* cfg `has_fmt_dbg_mutex_poisoned` from tests

### Fixed
* test_pipeio::test_size on rustc(1.67.0-beta.6)

## [0.3.13] (2023-01-05)
### Changed
* reformat `CHANGELOG.md`

### Fixed
* clippy: `Box::new(_)` of default value

## [0.3.12] (2023-01-02)
### Fixed
* test: struct size checking per rust version

## [0.3.11] (2022-06-13)
### Changed
* changes to edition 2021

## [0.3.10] (2022-05-21)
### Fixed
* bug : test_pipeio::test_size, test_stringio::test_size

## [0.3.9] (2021-11-14)
### Added
* add more documents

### Changed
* clean source codes

## [0.3.8] (2021-09-10)
### Changed
* update crates: criterion(0.3.5)

## [0.3.7] (2021-06-24)
### Added
* add a rustc 1.53.0 support cfg to test and build.rs

### Changed
* update depends

## [0.3.6] (2021-04-06)
### Added
* add: impl std::io::Read for &dyn runnel::StreamIn
* add: impl std::io::Write for &dyn runnel::StreamOut
* add: impl std::io::Write for &dyn runnel::StreamErr

## [0.3.5] (2021-04-04)
### Added
* add: attribute `#[inline(always)]`

### Changed
* update depends

## [0.3.4] (2021-03-08)
### Changed
* update crate: rustc_version ("0.3")

## [0.3.3] (2021-03-08)
### Added
* add bench

### Changed
* change pipeio auto flush from '\n' buffer to fix size buffer for
  good performance. This makes it faster than the Linux command pipe line.

## [0.3.2] (2021-03-07)
### Changed
* change in pipeio, Receiver<String> to Receiver<Vec<u8>>
* change in pipeio, Sender<String> to Sender<Vec<u8>>

## [0.3.1] (2021-03-03)
### Added
* add: auto flush to pipeio RawPipeOut::write().

## [0.3.0] (2021-02-21)
### Added
* add: RunnelIoeBuilder and set StreamIoe field private
* add: fn fill_stringio_with_str() into RunnelIoeBuilder

### Changed
* rename StreamIoe to RunnelIoe

### Removed
* remove call flush() in StreamIoe::drop(), cause of lock-up

## [0.2.2] (2021-02-20)
### Fixed
* miss: io::Error process of fn medium::RawPipeOut::flush()

## [0.2.1] (2021-02-19)
### Fixed
* bug: add call flush() in StreamIoe::drop()

## [0.2.0] (2021-02-14)
### Added
* add doc
* add trait std::fmt::Debug to struct StreamIoe

### Changed
* change pub to private: medium::PipeIn, medium::StringIn, ...
* rename private medium::PipeIn to medium::LockablePipeIn, ...
* rename medium::StreamInPipeIn to medium::PipeIn, ...
* rename medium::StreamInLockPipeIn to medium::PipeInLock, ...
* rename StreamIoe.sin to StreamIoe.pin

## [0.1.4] (2021-02-05)
### Fixed
* dox in Cargo.toml

## [0.1.3] (2021-02-05)
### Fixed
* doc

## [0.1.2] (2021-01-24)
### Added
* add cfg(has_fat_stdout) and test support before rustc 1.44.0
* add pipeio to streamio crate
* add tests with stream module

### Changed
* rename streamio to runnel

## [0.1.0] (2021-01-17)
* first commit

[Unreleased]: https://github.com/aki-akaguma/runnel/compare/v0.3.19..HEAD
[0.3.19]: https://github.com/aki-akaguma/runnel/compare/v0.3.18..v0.3.19
[0.3.18]: https://github.com/aki-akaguma/runnel/compare/v0.3.17..v0.3.18
[0.3.17]: https://github.com/aki-akaguma/runnel/compare/v0.3.16..v0.3.17
[0.3.16]: https://github.com/aki-akaguma/runnel/compare/v0.3.15..v0.3.16
[0.3.15]: https://github.com/aki-akaguma/runnel/compare/v0.3.14..v0.3.15
[0.3.14]: https://github.com/aki-akaguma/runnel/compare/v0.3.13..v0.3.14
[0.3.13]: https://github.com/aki-akaguma/runnel/compare/v0.3.12..v0.3.13
[0.3.12]: https://github.com/aki-akaguma/runnel/compare/v0.3.11..v0.3.12
[0.3.11]: https://github.com/aki-akaguma/runnel/compare/v0.3.10..v0.3.11
[0.3.10]: https://github.com/aki-akaguma/runnel/compare/v0.3.9..v0.3.10
[0.3.9]: https://github.com/aki-akaguma/runnel/compare/v0.3.8..v0.3.9
[0.3.8]: https://github.com/aki-akaguma/runnel/compare/v0.3.7..v0.3.8
[0.3.7]: https://github.com/aki-akaguma/runnel/compare/v0.3.6..v0.3.7
[0.3.6]: https://github.com/aki-akaguma/runnel/compare/v0.3.5..v0.3.6
[0.3.5]: https://github.com/aki-akaguma/runnel/compare/v0.3.4..v0.3.5
[0.3.4]: https://github.com/aki-akaguma/runnel/compare/v0.3.3..v0.3.4
[0.3.3]: https://github.com/aki-akaguma/runnel/compare/v0.3.2..v0.3.3
[0.3.2]: https://github.com/aki-akaguma/runnel/compare/v0.3.1..v0.3.2
[0.3.1]: https://github.com/aki-akaguma/runnel/compare/v0.3.0..v0.3.1
[0.3.0]: https://github.com/aki-akaguma/runnel/compare/v0.2.2..v0.3.0
[0.2.2]: https://github.com/aki-akaguma/runnel/compare/v0.2.1..v0.2.2
[0.2.1]: https://github.com/aki-akaguma/runnel/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/runnel/compare/v0.1.4..v0.2.0
[0.1.4]: https://github.com/aki-akaguma/runnel/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/runnel/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/runnel/releases/tag/v0.1.2

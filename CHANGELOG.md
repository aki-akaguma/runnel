# Changelog: runnel
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.3] - 2026-05-27
### Changed
- Update crates in xbench: `clf` (0.2), `criterion` (0.8).

## [0.4.2] - 2026-05-15
### Fixed
- Address issues in `test_size_of()`.

## [0.4.1] - 2026-05-15
### Added
- `LockAny` internal trait to consistently handle mutex locking. This prevents panic propagation when a lock is poisoned by instead returning the inner data.

### Fixed
- Refactor `RawStringIn` and `RawPipeIn` to strictly adhere to the `BufRead` contract. `fill_buf` no longer advances the internal position; `consume` is now responsible for pointer advancement.
- Improve error handling in `RawPipeIn` and `RawLinePipeIn`: they now gracefully return EOF or `None` instead of panicking when the sender is closed.
- Refactor `lines()` method in `StringIn`, `PipeIn`, and `LinePipeIn` to be non-destructive. Previously, calling `lines()` a second time would cause a panic.

### Changed
- Optimize `RawPipeOut::flush` and `RawLinePipeOut::flush_line` by using `std::mem::take` to move the buffer into the channel, eliminating unnecessary clones, allocations, and manual element transfers.
- Simplify locking logic by removing `Option` wrapping for internal `BufReader` in `medium` implementations.
- Standardize the use of `#[inline]` across the library, replacing `#[inline(always)]` to allow better compiler optimization heuristics.

## [0.4.0] - 2025-08-19
### Added
- `specs` documentation.
- `medium::linepipeio` providing 3x performance over `medium::pipeio`.

### Changed
- Redesign the core interface for improved usability.

## [0.3.19] - 2024-06-19
### Changed
- Switch conditional compilation from `rust_version` to `rustversion`.

### Fixed
- Resolve `clippy::dead_code` warnings.

## [0.3.18] - 2024-06-09
### Changed
- Rename `config` to `config.toml`.
- Update crates: `criterion` (0.5).
- Support Rust 1.60.0 in GitHub workflows for testing and building.

## [0.3.17] - 2023-02-12
### Added
- GitHub workflows for Ubuntu, macOS, and Windows.
- Test status badges in `README.tpl`.

### Changed
- Refactor `Makefile`.

### Removed
- `COPYING` file.

### Fixed
- Update `LICENSE-APACHE` and `LICENSE-MIT`.
- Resolve `clippy::box_default` warnings.

## [0.3.16] - 2023-01-28
### Added
- GitHub workflow for testing.
- Test status badges in `README.tpl`.

### Fixed
- Update `Makefile` to use Rustc 1.66.1 instead of 1.66.0.
- Resolve `clippy::box_default` warnings.
- Update `LICENSE` files.
- Resolve `test_size_of()` failures on macOS and Windows.

## [0.3.15] - 2023-01-10
### Added
- Version difference links to `CHANGELOG.md`.
- Badges in `README.tpl`.

### Changed
- Move benchmarks into `xbench`.

## [0.3.14] - 2023-01-06
### Added
- Specify `rust-version = "1.57.0"` in `Cargo.toml`.
- `all-test-version` target to `Makefile`.

### Removed
- `has_fat_stdout` cfg from tests.
- `has_fmt_dbg_mutex_poisoned` cfg from tests.

### Fixed
- Resolve `test_pipeio::test_size` failure on Rustc 1.67.0-beta.6.

## [0.3.13] - 2023-01-05
### Changed
- Reformat `CHANGELOG.md`.

### Fixed
- Resolve `clippy::Box::new(_)` warnings for default values.

## [0.3.12] - 2023-01-02
### Fixed
- Ensure struct size checking in tests is correct per Rust version.

## [0.3.11] - 2022-06-13
### Changed
- Update to Rust 2021 edition.

## [0.3.10] - 2022-05-21
### Fixed
- Resolve issues in `test_pipeio::test_size` and `test_stringio::test_size`.

## [0.3.9] - 2021-11-14
### Added
- Documents.

### Changed
- Clean up source code.

## [0.3.8] - 2021-09-10
### Changed
- Update crates: `criterion` (0.3.5).

## [0.3.7] - 2021-06-24
### Added
- Rustc 1.53.0 support cfg to tests and `build.rs`.

### Changed
- Update dependencies.

## [0.3.6] - 2021-04-06
### Added
- `std::io::Read` implementation for `&dyn runnel::StreamIn`.
- `std::io::Write` implementation for `&dyn runnel::StreamOut`.
- `std::io::Write` implementation for `&dyn runnel::StreamErr`.

### Changed
- Update dependencies.

## [0.3.5] - 2021-04-04
### Added
- Use of `#[inline(always)]` attribute.

### Changed
- Update dependencies.

## [0.3.4] - 2021-03-08
### Changed
- Update crate: `rustc_version` (0.3).

## [0.3.3] - 2021-03-08
### Added
- Benchmarks.

### Changed
- Improve `pipeio` performance by switching auto-flush from `\n` buffer to fixed-size buffer, exceeding Linux command pipeline speed.

## [0.3.2] - 2021-03-07
### Changed
- Update `pipeio` to use `Receiver<Vec<u8>>` and `Sender<Vec<u8>>` instead of `String`.

## [0.3.1] - 2021-03-03
### Added
- Auto-flush for `pipeio` in `RawPipeOut::write()`.

## [0.3.0] - 2021-02-21
### Added
- `RunnelIoeBuilder` and make `StreamIoe` fields private.
- `fill_stringio_with_str()` in `RunnelIoeBuilder`.

### Changed
- Rename `StreamIoe` to `RunnelIoe`.

### Removed
- `flush()` call in `StreamIoe::drop()` to prevent lock-ups.

## [0.2.2] - 2021-02-20
### Fixed
- Correct `io::Error` processing in `medium::RawPipeOut::flush()`.

## [0.2.1] - 2021-02-19
### Fixed
- Ensure `flush()` is called in `StreamIoe::drop()`.

## [0.2.0] - 2021-02-14
### Added
- Documentation.
- `std::fmt::Debug` implementation for `RunnelIoe`.

### Changed
- Visibility of `medium` internal types to private.
- Rename internal `medium` types for clarity (e.g., `PipeIn` to `LockablePipeIn`).
- Rename `StreamIoe.sin` to `StreamIoe.pin`.

## [0.1.4] - 2021-02-05
### Fixed
- Correct `dox` configuration in `Cargo.toml`.

## [0.1.3] - 2021-02-05
### Fixed
- Improve documentation.

## [0.1.2] - 2021-01-24
### Added
- `cfg(has_fat_stdout)` and test support for Rustc versions before 1.44.0.
- `pipeio` to `streamio` crate.
- Tests for the stream module.

### Changed
- Rename project from `streamio` to `runnel`.

## [0.1.0] - 2021-01-17
- Initial release.

[Unreleased]: https://github.com/aki-akaguma/runnel/compare/v0.4.3..HEAD
[0.4.3]: https://github.com/aki-akaguma/runnel/compare/v0.4.2..v0.4.3
[0.4.2]: https://github.com/aki-akaguma/runnel/compare/v0.4.1..v0.4.2
[0.4.1]: https://github.com/aki-akaguma/runnel/compare/v0.4.0..v0.4.1
[0.4.0]: https://github.com/aki-akaguma/runnel/compare/v0.3.19..v0.4.0
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

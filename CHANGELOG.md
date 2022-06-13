runnel TBD
===
Unreleased changes. Release notes have not yet been written.

* changes to edition 2021

0.3.10 (2022-05-21)
=====

* bug fix: test_pipeio::test_size, test_stringio::test_size

0.3.9 (2021-11-14)
=====

* clean source codes
* add more documents

0.3.8 (2021-09-10)
=====

* update crates: criterion(0.3.5)

0.3.7 (2021-06-24)
=====

* update depends
* add a rustc 1.53.0 support cfg to test and build.rs

0.3.6 (2021-04-06)
=====

* add: impl std::io::Read for &dyn runnel::StreamIn
* add: impl std::io::Write for &dyn runnel::StreamOut
* add: impl std::io::Write for &dyn runnel::StreamErr

0.3.5 (2021-04-04)
=====

* add: attribute `#[inline(always)]`
* update depends

0.3.4 (2021-03-08)
=====

* update crate: rustc_version ("0.3")

0.3.3 (2021-03-08)
=====

* change pipeio auto flush from '\n' buffer to fix size buffer for
  good performance. This makes it faster than the Linux command pipe line.
* add bench

0.3.2 (2021-03-07)
=====

* change in pipeio, Receiver<String> to Receiver<Vec<u8>>
* change in pipeio, Sender<String> to Sender<Vec<u8>>

0.3.1 (2021-03-03)
=====

* add: auto flush to pipeio RawPipeOut::write().

0.3.0 (2021-02-21)
=====

* add: RunnelIoeBuilder and set StreamIoe field private
* rename StreamIoe to RunnelIoe
* add: fn fill_stringio_with_str() into RunnelIoeBuilder
* remove call flush() in StreamIoe::drop(), cause of lock-up

0.2.2 (2021-02-20)
=====

* fix miss: io::Error process of fn medium::RawPipeOut::flush()

0.2.1 (2021-02-19)
=====

* fix bug: add call flush() in StreamIoe::drop()

0.2.0 (2021-02-14)
=====

* add doc
* change pub to private: medium::PipeIn, medium::StringIn, ...
* rename private medium::PipeIn to medium::LockablePipeIn, ...
* rename medium::StreamInPipeIn to medium::PipeIn, ...
* rename medium::StreamInLockPipeIn to medium::PipeInLock, ...
* add trait std::fmt::Debug to struct StreamIoe
* rename StreamIoe.sin to StreamIoe.pin

0.1.4 (2021-02-05)
=====

* fix dox in Cargo.toml

0.1.3 (2021-02-05)
=====

* fix doc

0.1.2 (2021-01-24)
=====

* add cfg(has_fat_stdout) and test support before rustc 1.44.0
* add pipeio to streamio crate
* rename streamio to runnel
* add tests with stream module

0.1.0 (2021-01-17)
=====
first commit

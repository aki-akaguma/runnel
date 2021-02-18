TBD
===
Unreleased changes. Release notes have not yet been written.

0.2.2 (2021-02-20)
=====

* fix miss: io::Error process of fn medium::RawPipeOut::flush()

0.2.1 (2021-02-19)
=====

* fix bug: add call flush() in StreamIoe::drop()

0.2.0 (2021-02-14)
=====
Feature:

* add doc
* change pub to private: medium::PipeIn, medium::StringIn, ...
* rename private medium::PipeIn to medium::LockablePipeIn, ...
* rename medium::StreamInPipeIn to medium::PipeIn, ...
* rename medium::StreamInLockPipeIn to medium::PipeInLock, ...
* add trait std::fmt::Debug to struct StreamIoe
* rename StreamIoe.sin to StreamIoe.pin

0.1.4 (2021-02-05)
=====
Feature:

* fix dox in Cargo.toml

0.1.3 (2021-02-05)
=====
Feature:

* fix doc

0.1.2 (2021-01-24)
=====
Feature:

* add cfg(has_fat_stdout) and test support before rustc 1.44.0
* add pipeio to streamio crate
* rename streamio to runnel
* add tests with stream module

0.1.0 (2021-01-17)
=====
first commit

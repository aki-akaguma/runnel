
all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test --offline

test-no-default-features:
	cargo test --offline --no-default-features

miri:
	cargo +nightly miri test --offline

clean:
	@cargo clean
	@rm -f z.*

clippy:
	cargo clippy --offline --tests --workspace

fmt:
	cargo fmt

doc:
	cargo doc

tarpaulin:
	cargo tarpaulin --offline --engine llvm --out html --output-dir ./target

bench:
	cargo xbench --bench=bench-pipeio -- --noplot

bench-clean:
	@rm -fr target/criterion


rustc_vers = 1.60.0

#rustc_vers = 1.60.0 1.61.0 1.62.1 1.63.0 1.64.0 1.65.0 1.66.1 1.67.1 1.68.1 1.69.0 \
	1.70.0 1.71.1 1.72.1 1.73.0 1.74.1 1.75.0 1.76.0 1.77.2 1.78.0 1.79.0 \
	1.80.0 1.81.0 1.82.0 1.83.0 1.84.1 1.85.1 1.86.0.1.87.0 1.88.0.1.89.0

target_base = x86_64-unknown-linux i686-unknown-linux i586-unknown-linux
target_base_2 = x86_64-unknown-linux-gnu x86_64-unknown-linux-musl \
	i686-unknown-linux-gnu i686-unknown-linux-musl i586-unknown-linux-gnu
#target_base = i586-unknown-linux

define test-rustc-templ =
target/stamp/stamp.test-rustc.$(1).$(2):
	@mkdir -p target/stamp
	cargo +$(1) test --target=$(2)-gnu
	cargo +$(1) test --target=$(2)-musl
	@touch target/stamp/stamp.test-rustc.$(1).$(2)
endef
define test-rustc-templ-2 =
target/stamp/stamp.test-rustc.$(1).$(2):
	@mkdir -p target/stamp
	cargo +$(1) test --target=$(2)
	@touch target/stamp/stamp.test-rustc.$(1).$(2)
endef

test-all-version: $(foreach ver,$(rustc_vers),$(foreach tb,$(target_base),target/stamp/stamp.test-rustc.$(ver).$(tb)))

test-clean:
	@rm -fr target/stamp

$(foreach ver,$(rustc_vers),$(eval $(foreach tb,$(target_base),$(eval $(call test-rustc-templ,$(ver),$(tb))))))

$(foreach ver,$(rustc_vers_2),$(eval $(foreach tb,$(target_base_2),$(eval $(call test-rustc-templ-2,$(ver),$(tb))))))

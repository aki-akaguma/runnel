// build.rs
use rustc_version as rs_v;

fn main() {
    let rt_version = rs_v::version().unwrap();
    //
    if rt_version < rs_v::Version::parse("1.44.0").unwrap() {
        println!("cargo:rustc-cfg=has_fat_stdout");
    }
    if rt_version >= rs_v::Version::parse("1.53.0").unwrap() {
        println!("cargo:rustc-cfg=has_fmt_dbg_mutex_poisoned");
    }
}

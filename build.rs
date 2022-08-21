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
    if rt_version >= rs_v::Version::parse("1.62.0-alpha").unwrap() {
        if rt_version >= rs_v::Version::parse("1.64.0-alpha").unwrap() {
            println!("cargo:rustc-cfg=has_ge_version_1_64");
        } else {
            println!("cargo:rustc-cfg=has_ge_version_1_62");
        }
    } else {
            println!("cargo:rustc-cfg=has_lt_version_1_62");
    }
}

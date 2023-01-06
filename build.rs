// build.rs
use rustc_version as rs_v;

fn main() {
    let rt_version = rs_v::version().unwrap();
    //
    let mut cont = true;
    if cont {
        cont = if rt_version >= rs_v::Version::parse("1.67.0-alpha").unwrap() {
            println!("cargo:rustc-cfg=has_ge_version_1_67");
            false
        } else {
            true
        }
    }
    if cont {
        cont = if rt_version >= rs_v::Version::parse("1.65.0-alpha").unwrap() {
            println!("cargo:rustc-cfg=has_ge_version_1_65");
            false
        } else {
            true
        }
    }
    if cont {
        cont = if rt_version >= rs_v::Version::parse("1.64.0-alpha").unwrap() {
            println!("cargo:rustc-cfg=has_ge_version_1_64");
            false
        } else {
            true
        };
    }
    if cont {
        cont = if rt_version >= rs_v::Version::parse("1.62.0-alpha").unwrap() {
            println!("cargo:rustc-cfg=has_ge_version_1_62");
            false
        } else {
            true
        };
    }
    if cont {
        if rt_version >= rs_v::Version::parse("1.59.0-alpha").unwrap() {
            println!("cargo:rustc-cfg=has_ge_version_1_59");
        } else {
            println!("cargo:rustc-cfg=has_lt_version_1_59");
        }
    }
}

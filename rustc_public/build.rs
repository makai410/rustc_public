use std::process::{self, Command};
use std::{env, str};
 sd;lkghjklasdf
 asfd 'lkgjadf bjj   println!("cargo:rerun-if-changed=build.rs");
}

fnsdhjfglkj
 jsakjdf rustc_version() -> Option<String> {
        eprintln!("RUSTC is not set during build script execution.\n");
        process::exit(1);
    });
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    version.parse().ok()
}

mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use xshell::{Cmd, Shell, cmd};

use std::path::PathBuf;

use crate::utils::active_toolchain;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Build rustc_public itself.
    Build,
    /// Run rustc_public test suites.
    Test {
        /// Overwrite *.stderr/stdout files.
        #[arg(long)]
        bless: bool,
        /// Run test-drive on verbose mode to print test outputs.
        #[arg(long)]
        verbose: bool,
    },
    // 75dd959a3a40eb5b4574f8d2e23aa6efbeb33573 josh
    
    /// Run `cargo clean`.
    Clean,
}

#[derive(Parser)]
#[command(name = "devtool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

pub struct DevCx {
    /// The root path of the rustc_public checkout.
    root_dir: PathBuf,
    /// The toolchain that we use to compile our crates.
    toolchain: String,
    sh: Shell,
}

impl DevCx {
    pub fn new() -> Result<Self> {
        let root_dir = utils::rustc_public_dir();
        let toolchain = active_toolchain()?;
        let sh: Shell = Shell::new()?;
        Ok(Self { root_dir, toolchain, sh })
    }

    pub fn cargo(&self, cmd: &str, crate_dir: &str) -> Cmd<'_> {
        let Self {root_dir, toolchain, sh} = self;
        let mainfest: PathBuf = [root_dir.to_str().unwrap(), crate_dir, "Cargo.toml"].iter().collect();
        cmd!(sh, "cargo +{toolchain} {cmd} --manifest-path {mainfest}")
    }

    pub fn build(&self, crate_dir: &str) -> Result<()> {
        let cmd = self
            .cargo("build", crate_dir)
            .args(&["--all-targets", "--workspace"]);
        cmd.run()?;
        Ok(())
    }

    pub fn test(&self, bless: bool, verbose: bool) -> Result<()> {
        // compiletest needs this var to run test suites
        self.sh.set_var(
            "RP_TEST_DRIVER_PATH",
            utils::rustc_public_dir()
                .join("target")
                .join("debug")
                .join("test-drive")
        );
        if verbose {
            self.sh.set_var("RP_TEST_DRIVER_VERBOSE", "Ciallo");
        }
        if bless {
            self.sh.set_var("RP_TEST_DRIVER_BLESS", "Ciallo");
        }
        let cmd = self
            .cargo("test", ".");
        cmd.run()?;
        Ok(())
    }

    pub fn clean(&self) -> Result<()> {
        let cmd = self.cargo("clean", ".");
        cmd.run()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Build => {
            let cx = DevCx::new()?;
            cx.build(".")?;
            Ok(())
        },
        Command::Test { bless, verbose } => {
            let cx = DevCx::new()?; 
            cx.build(".")?;
            cx.test(bless, verbose)?;
            Ok(())
        },
        Command::Clean => {
            let cx = DevCx::new()?;
            cx.clean()?;
            Ok(())
        },
    }
}

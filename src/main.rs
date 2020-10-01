mod handler;
mod nrsync;
#[macro_use]
extern crate log;

use clap::Clap;
use env_logger::{Builder, Target};
use std::env;

macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[derive(Clap)]
#[clap(version= crate_version!(), author = "Kavashen Pather")]
pub struct Opts {
    /// New Relic Account ID
    #[clap(short = "a", env = "NR_ACCOUNT_ID")]
    account_id: String,
    /// New Relic One API Key
    #[clap(short = "k", env = "NR_API_KEY")]
    api_key: String,
    /// NerdPack UUID of the Integrations Manager
    #[clap(short = "u", env = "NR_UUID")]
    uuid: String,
    /// Config Collection to sync with
    #[clap(short = "c", env = "NR_COLLECTION")]
    collection: String,
    /// Verbose logging
    #[clap(short, env = "VERBOSE")]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    if opts.verbose {
        env::set_var("RUST_LOG", "debug")
    }

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stderr); // use stderr so infra agent doesn't complain
    builder.init();

    nrsync::start(opts).await;
}

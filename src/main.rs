use core::panic;

use clap::Parser;
use xdiff::{cli::{Action, Args, RunArgs}, DiffConfig};
use anyhow::{anyhow, Ok, Result};


/// cargo run -- run -p rust -c fixtures/test.yml -e a=100
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Run(args) => run(args).await?,
        _ => panic!("Not implemented"),
    }
    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    let config_file = args.config.unwrap_or_else(|| "./xdiff.yml".to_string());
    let config = DiffConfig::load_yml(&config_file).await?; 
    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow!("Profile {} not found in config file {}",
         args.profile,
         config_file
        )
    })?;

    let extra_args = args.extra_params.into();
    let diff_result = profile.diff(extra_args);
    Ok(())
}

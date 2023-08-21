mod cmd;
mod op;

use anyhow::Result;
use clap::Parser;
use cmd::Args;
use op::handle_args;

fn main() -> Result<()> {
    let args = Args::parse();
    handle_args(args)?;
    Ok(())
}

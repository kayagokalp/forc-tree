use clap::Parser;

/// A forc plugin to generate dependency graph of given Sway project.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path of the Sway project.
    #[arg(short, long)]
    pub path: Option<String>,

    /// Try to generate dependency graph without internet connection.
    #[arg(short, long)]
    pub offline: bool,
}

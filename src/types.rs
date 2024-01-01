use clap::Parser;
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to file
    pub path: String,
}

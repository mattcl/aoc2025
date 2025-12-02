#[cfg(not(feature = "lite"))]
mod cli;

#[cfg(feature = "lite")]
mod fast_cli;

#[cfg(not(feature = "lite"))]
pub fn main() -> Result<(), anyhow::Error> {
    cli::Cli::run()
}

#[cfg(feature = "lite")]
pub fn main() -> Result<(), anyhow::Error> {
    fast_cli::run()
}

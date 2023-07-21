
#[macro_use] extern crate log;

use clap::Parser;

use cassowary::logging;

/// Cassowary inventory controller
#[derive(Debug, Parser)]
#[clap(name = "Sap Confirmation Files")]
#[clap(author, version)]
struct Args {
    /// Logging verbosity
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let _ = logging::init_logging(args.verbose.log_level_filter());
    trace!("Logging initialized");

    Ok(())
}
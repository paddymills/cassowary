
use clap::Parser;

use cassowary_core::logging;

/// Cassowary inventory controller
#[derive(Debug, Parser)]
#[clap(name = "Cassowary")]
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
    log::trace!("Logging initialized");

    Ok(())
}

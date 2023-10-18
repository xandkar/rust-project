use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long, default_value_t = false)]
    debug: bool,

    #[clap(short, long, default_value = "logs")]
    log_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let _logger_guard = foo::logger::init(cli.debug, &cli.log_dir)?;
    tracing::info!("Starting with cli: {:?}", &cli);
    tracing::debug!("Doing stuff");
    tracing::trace!("Tracing things");
    tracing::info!("Exiting");
    Ok(())
}

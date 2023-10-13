use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long, default_value_t = true)]
    debug: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    foo::logger::init(cli.debug)?;
    tracing::debug!("cli: {:?}", &cli);
    Ok(())
}

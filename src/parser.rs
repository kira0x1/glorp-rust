use clap::Parser;
use tracing::Level;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// set development mode for less clutter in log and only show useful info
    #[arg(short, long, default_value_t = false)]
    pub dev: bool,
}

// config parsed from args
pub struct DevConfig {
    pub is_dev: bool,
    pub trace_level: Level,
}

// Returns Config
pub fn parse_args() -> DevConfig {
    let args = Args::parse();

    DevConfig {
        is_dev: args.dev,
        trace_level: if args.dev { Level::INFO } else { Level::TRACE },
    }
}
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Set whether the fireworks show will loop infinitely
    #[arg(short, long)]
    pub looping: bool,

    /// Set whether the fireworks will have color gradient
    ///
    /// If this is enabled, it is recommanded that your terminal is non-transparent and has black bg color to get better visual effects
    #[arg(short, long)]
    pub gradient: bool,

    /// Select which demo to run
    #[arg(short, long, value_name = "DEMO-NUMBER")]
    pub demo: u8,
}

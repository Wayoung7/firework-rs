use clap::Parser;

/// Used to receive command line arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Select which demo to run. (optional)
    ///
    /// If this is not specified, automatically run the infinite random firework demo
    #[arg(short, long, value_name = "DEMO-NUMBER")]
    pub demo: Option<u8>,

    /// Set whether the fireworks show will loop infinitely
    #[arg(short, long)]
    pub looping: bool,

    /// Set whether the fireworks will have color gradient
    ///
    /// If this is enabled, it is recommanded that your terminal is non-transparent and has black bg color to get better visual effects
    #[arg(short, long)]
    pub gradient: bool,

    /// Set frame per second
    ///
    /// If this is not specified, the default fps is 12
    #[arg(long, value_name = "FRAME-RATE")]
    pub fps: Option<u8>,
}

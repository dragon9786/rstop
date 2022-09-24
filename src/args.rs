use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    author = "Govind<dragon9786@gmail.com",
    version = "0.1.0",
    about = "A better htop"
)]
pub struct RsTopArgs {
    /// Disable color
    #[clap(short, long, default_value_t = false)]
    pub no_color: bool,

    /// Interval between refreshes in seconds
    #[clap(short, long, default_value_t = 2.0)]
    pub interval: f64,
}

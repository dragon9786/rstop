use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    author = "Govind<dragon9786@gmail.com",
    version = "0.0.2",
    about = "A better htop"
)]
#[repr(C)]
pub struct RsTopArgs {
    /// Disable color
    #[clap(short, long, default_value_t = false)]
    pub no_color: bool,

    /// Interval between refreshes in seconds
    #[clap(short, long, default_value_t = 2.0)]
    pub interval: f64,

    /// Show only the given PIDs (Optional)
    #[clap(short, long)]
    pub pid: Option<u32>,

    /// Show only processes for a given user (Optional)
    #[clap(short, long)]
    pub user_id: Option<u32>,
}

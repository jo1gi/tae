use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Arguments {
    /// Links to open
    pub urls: Vec<String>,
    /// Path to config file
    #[structopt(short, long, default_value = "tae.toml")]
    pub config: std::path::PathBuf,
    /// Dry mode (Don't run command)
    #[structopt(long)]
    pub dry: bool,
    /// Don't show command output in terminal
    #[structopt(short="H", long)]
    pub hide_output: bool,
}

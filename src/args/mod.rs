use clap::Parser;

#[derive(Parser, Debug)]
#[non_exhaustive]
pub struct Args {
    // if psh runs in netdata_plugin mode?
    #[clap(skip)]
    pub netdata_plugin: Option<bool>,

    /// frequency that's passed by netdata if run as netdata plugin.
    pub netdata_freq: u64,

    #[arg(verbatim_doc_comment)]
    /// Path to the install script
    #[arg(long)]
    #[arg(value_name = "PATH")]
    pub install: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Path to the get_sysinfo script
    #[arg(long)]
    #[arg(value_name = "PATH")]
    pub get_sysinfo: Option<String>,
}

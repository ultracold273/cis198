
use structopt::StructOpt;
// use std::ffi::CString;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "stracer", 
    version = "0.1.0",
    about = "A simple stracer written in Rust",
    rename_all = "snake"
)]
pub struct Opt {
    #[structopt(short, long, conflicts_with = "to_trace", name = "dont_trace")]
    pub dont_trace: Option<Vec<String>>,

    #[structopt(short, long, conflicts_with = "dont_trace", name = "to_trace")]
    pub to_trace: Option<Vec<String>>,

    pub exe: String,

    #[structopt(raw(true))]
    pub exe_args: Vec<String>,
}

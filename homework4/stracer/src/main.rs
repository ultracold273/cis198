
mod args;
mod trace;
mod util;
mod system_call_names;

use structopt::StructOpt;
use args::Opt;
use trace::*;
use std::ffi::CString;

fn main() {
    env_logger::init();
    let mut opt = Opt::from_args();
    println!("{:?}", opt);

    opt.exe_args.insert(0, opt.exe.clone());
    let exe_c = CString::new(opt.exe).unwrap();
    let exe_args = opt.exe_args.iter()
                               .map(|s| CString::new(s.as_str()).unwrap())
                               .collect::<Vec<CString>>();
    trace(&exe_c, &exe_args).unwrap();
}

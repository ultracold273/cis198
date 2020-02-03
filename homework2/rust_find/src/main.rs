use std::path::PathBuf;
use std::fs::{File, ReadDir};
use std::io::BufReader;
use std::io::prelude::*;
use structopt::StructOpt;
use regex::Regex;

#[allow(dead_code)]
fn get_matches(paths: &Vec<PathBuf>, pattern: &Regex) -> Vec<PathBuf> {
    paths.iter()
      .filter(|s| pattern.is_match(s.to_str().unwrap()))
      .map(|s| s.clone())
      .collect()
}

#[test]
fn get_matches_tests() {
    let paths: Vec<PathBuf> = vec![
        PathBuf::from("../temp/homework1/src/lib3.rs"),
        PathBuf::from("../temp/homework1/src/lib2.rs"),
        PathBuf::from("../temp/homework1/src/lib.rs"),
        PathBuf::from("../homework1/lib3.rs"),
        PathBuf::from("../homework1/lib2.rs"),
        PathBuf::from("../homework1/lib.rs"),
        PathBuf::from("../rust_find/src/main.rs")
    ];
    let fpaths: Vec<PathBuf> = vec![
        PathBuf::from("../temp/homework1/src/lib3.rs"),
        PathBuf::from("../temp/homework1/src/lib2.rs"),
        PathBuf::from("../temp/homework1/src/lib.rs"),
        PathBuf::from("../rust_find/src/main.rs")
    ];
    assert_eq!(get_matches(&paths, &Regex::new(r"src/.*\.rs").unwrap()), fpaths);
}

fn get_directories(dir: ReadDir) -> std::io::Result<Vec<PathBuf>> {
    let mut s = Vec::<PathBuf>::new();
    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let next_dir = std::fs::read_dir(path)?;
            let r = get_directories(next_dir)?;
            // for p in r { s.push(p); }
            s.extend(r);
        } else {
            s.push(path);
        }
    }
    Ok(s)
}

#[test]
fn get_directories_tests() {
    let dir = std::fs::read_dir("../../").unwrap();
    let directories = get_directories(dir).unwrap();
    for d in directories {
        println!("{:?}", d);
    }
}

fn parse_patterns(args: &Opt) -> Option<Vec<Regex>> {
    let mut rev = Vec::new();
    if let Some(ref patpb) = &args.patterns {
        let bread = BufReader::new(File::open(patpb).ok()?);
        for line in bread.lines() {
            match Regex::new(& line.ok()?) {
                Result::Ok(re) => rev.push(re),
                Result::Err(_) => {
                    if args.robust {
                        eprintln!("[Warning] malformed patterns.");
                        continue;
                    } else { return None }
                },
            }
        }
    }
    if let Some(ref patstr) = &args.pattern {
        let re = Regex::new(&patstr).ok()?;
        rev.push(re);
    }
    if rev.len() > 0 { Some(rev) } else { None }
}

fn parse_dirs(args: &Opt) -> Option<Vec<PathBuf>> {
    let mut pbv = Vec::new();
    if let Some(ref dirpb) = &args.dirs {
        let bread = BufReader::new(File::open(dirpb).ok()?);
        for line in bread.lines() {
            let pb = PathBuf::from(line.ok()?);
            if pb.is_dir() {
                pbv.push(pb);
            } else if args.robust {
                eprintln!("[Warning] invalid directory path")
            } else { return None }
        }
    }

    if let Some(ref dpb) = &args.dir {
        if dpb.is_dir() {
            pbv.push(dpb.clone());
        }
    }
    if pbv.len() > 0 { Some(pbv) } else { None }
}

fn parse_output(args: &Opt) -> std::io::Result<Option<File>> {
    let path = match args.output {
        Some(ref p) => p,
        None => return Ok(None),
    };
    File::create(path).map(|x| Some(x))
}

fn display(v: &Vec<PathBuf>, file: &mut Option<File>) {
    let sv: Vec<String> = v.iter().map(|pb| format!("{:?}", pb)).collect();
    let ostr = sv.join("\n");
    if let Some(f) = file {
        f.write_all(ostr.as_bytes());
    } else {
        print!("{}", ostr);
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rust-find", 
    version = "0.1.0",
    about = "A command line utility for searching for files with regexes"
)]
struct Opt {
    /// On encountering an error, continue running the program, 
    /// printing out a warning.
    #[structopt(short, long)]
    robust: bool,

    /// Directory to search in.
    #[structopt(short, long, parse(from_os_str), required_unless = "dirs-input")]
    // #[structopt(short, long, parse(from_os_str))]
    dir: Option<PathBuf>,

    /// Take directories from file instead of command line.
    #[structopt(long = "dirs-input", name = "dirs-input", parse(from_os_str))]
    dirs: Option<PathBuf>,

    /// Write results to output file instead of stdout.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// Pattern to use.
    #[structopt(short, long, required_unless = "patterns-input")]
    // #[structopt(short, long)]
    pattern: Option<String>,

    /// Take patterns from file instead of command line
    #[structopt(name = "patterns-input", long = "patterns-input", parse(from_os_str))]
    patterns: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);
    let patv = parse_patterns(&opt).unwrap();
    let pbv = parse_dirs(&opt).unwrap();
    let mut of = parse_output(&opt).unwrap();
    for pb in &pbv {
        let dir = std::fs::read_dir(pb).unwrap();
        let apbv = get_directories(dir).unwrap();
        for pat in &patv {
            let mvpb = get_matches(&apbv, pat);
            display(&mvpb, &mut of);
        }
    }
}
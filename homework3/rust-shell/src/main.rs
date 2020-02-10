mod raw_terminal;

use std::fmt;
use raw_terminal::RawTerminal;

use nix::unistd::execvp;
use nix::unistd::{fork, ForkResult, dup2, pipe};
use std::ffi::{CString, CStr};
use std::os::unix::io::AsRawFd;
use std::io::prelude::*;

const PROMT: &str = "> ";
// const PATH: &str = "PATH=/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin";

#[derive(Debug)]
struct CommandLine {
    command: Option<String>,
    args: Option<Vec<String>>,
    redir_fp: Option<String>,
}

fn print_promt(term: &mut RawTerminal) {
    term.write(PROMT);
}

fn readline(term: &mut RawTerminal) -> String {
    let mut cmd = String::new();
    loop {
        let rl = term.read_line();
        let s = rl.trim();
        if !s.ends_with(r"\") { 
            cmd += s;
            break; 
        } else {
            cmd += &s[0..s.len()-1];
        }
    }
    cmd
}

struct ParseError(String);

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    } 
}

fn parse_cmdline(cmd: String) -> Result<CommandLine, ParseError> {
    let mut cmdline = CommandLine {
        command: None,
        args: None,
        redir_fp: None,
    };
    let mut tokens: Vec<&str> = cmd.split(' ')
                               .filter(|s| *s != "")
                               .collect();
    if let Some(idx) = tokens.iter().position(|&s| s == ">") {
        if idx + 1 < tokens.len() {
            cmdline.redir_fp = tokens.get(idx + 1).map(|&s| s.to_string());
            tokens.remove(idx + 1);
            tokens.remove(idx);
        } else { return Err(ParseError("Error: parse error near \\n".to_string())) }
    }
    cmdline.command = tokens.get(0).map(|&s| s.to_string());
    cmdline.args = tokens.get(1..)
                         .map(|s| s.iter()
                                   .map(|&t| t.to_string())
                                   .collect()
                         );
    Ok(cmdline)
}

// fn make_out_term() -> Fn

fn exec(cmd: &CommandLine, term: &mut RawTerminal) -> nix::Result<()> {
    let (fdr, fdw) = pipe()?;
    match fork()? {
        ForkResult::Parent { .. } => {
            nix::unistd::close(fdw)?;
            let mut buf = [0u8; 128];
            if cmd.redir_fp.is_none() {
                while let Ok(rdsize) = nix::unistd::read(fdr, &mut buf) {
                    if rdsize == 0 { break }
                    let s = std::str::from_utf8(&buf[..rdsize]).unwrap();
                    term.write(s);
                }
            } else {
                let path = cmd.redir_fp.as_ref();
                let mut fd = std::fs::File::create(path.unwrap()).unwrap();
                while let Ok(rdsize) = nix::unistd::read(fdr, &mut buf) {
                    if rdsize == 0 { break }
                    fd.write(&buf).unwrap();
                }
            }
            nix::unistd::close(fdr)?;
        },
        ForkResult::Child => {
            // Actually use pipe to redirect output
            // If not with a file, redirect to the parent program to write
            // the terminal??
            // let mut (rfd1, rfd2) = pipe();
            // let mut rawfd: Option<RawFd> = None;
            // if let Some(ref path) = cmd.redir_fp {
            //     let fd = std::fs::File::create(path).unwrap();
            //     rawfd = Some(fd.as_raw_fd());
            //     dup2(rawfd.unwrap(), std::io::stdout().as_raw_fd()).unwrap();
            // }
            nix::unistd::close(fdr)?;
            dup2(fdw, std::io::stdout().as_raw_fd())?;
            nix::unistd::close(fdw)?;
            let ecmd = CString::new(cmd.command.as_ref().unwrap().clone()).unwrap();
            let mut eargs = cmd.args.as_ref().unwrap().iter()
                                .map(|s| CString::new(s.clone()).unwrap())
                                .collect::<Vec<CString>>();
            eargs.insert(0, ecmd.clone());
            let args = eargs.iter().map(|cs| cs.as_c_str()).collect::<Vec<&CStr>>();
            execvp(&ecmd, &args)?;
            // if rawfd.is_some() {
                // nix::unistd::close(rawfd.unwrap()).unwrap();
            // }
        },
    }
    Ok(())
}

fn main() {
    let mut terminal = raw_terminal::RawTerminal::new();
    loop {
        print_promt(&mut terminal);
        let rcmdl = parse_cmdline(readline(&mut terminal));
        if let Err(p) = rcmdl {
            terminal.writeln(&p.0);
            continue;
        }

        let cmdl = rcmdl.unwrap();
        
        match cmdl.command {
            None => continue,
            Some(ref cmd) => match &cmd[..] {
                // Built-in functions
                "quit" | "exit" => break,
                // _ => terminal.writeln(cmd),
                // "cd" | "chdir" => chdir(),
                _ => {},
            }
        }

        exec(&cmdl, &mut terminal).unwrap();
        terminal.move_to_beginning_of_line();
    }
}
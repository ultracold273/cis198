
use crate::system_call_names::*;
use crate::util;
use std::ffi::CString;
use std::collections::{HashSet, HashMap};
use log::info;
use nix::sys::{wait, ptrace, signal};
use nix::unistd::*;
use libc::{c_void, user_regs_struct};

const ADDRESS_LOWER_BOUND: i64 = 10000;
const DEBUG_PTRACE_EVENT: [&str; 7] = [
    "PTRACE_EVENT_FORK", "PTRACE_EVENT_VFORK", "PTRACE_EVENT_CLONE",
    "PTRACE_EVENT_EXEC", "PTRACE_EVENT_VFORK_DONE", "PTRACE_EVENT_EXIT",
    "PTRACE_EVENT_SECCOMP"];

pub fn trace_prehook(regs: &user_regs_struct, pid: Pid) -> String {
    let name = SYSTEM_CALL_NAMES[regs.orig_rax as usize].to_string();
    let print_string = vec!["execve", "access", "stat", "lstat", "chdir"];
    if print_string.contains(&name.as_str()) {
        let arg_str = util::read_string(regs.rdi as *mut c_void, pid);
        format!("{}(\"{}\")", name, arg_str)
    } else {
        format!("{}()", name)
    }
}

pub fn trace_posthook(regs: &user_regs_struct) -> String {
    let retval = regs.rax as i64;
    if retval > ADDRESS_LOWER_BOUND {
        format!("{:#x}", retval)
    } else {
        format!("{}", retval)
    }
}

pub fn trace(prog: &CString, args: &[CString]) -> nix::Result<()> {
    match fork()? {
        ForkResult::Parent { child, .. } => {
            wait::waitpid(child, None)?;
            util::ptrace_set_options(child)?;
            ptrace::syscall(child)?;

            // let mut line: Vec<String> = vec![];
            let mut live_process = HashSet::new();
            live_process.insert(child);
            let mut proc_hook = HashMap::new();
            proc_hook.insert(child, (true, Vec::<String>::new()));
            loop {
                // let actual_pid = match wait::waitpid(child, None)? {
                let actual_pid = match wait::wait()? {
                    wait::WaitStatus::Exited(pid, code) => {
                        info!("[{}] Process exit normally with code {}", pid, code);
                        let (_, line) = proc_hook.get_mut(&pid).unwrap();
                        line.push("Process finished!".to_string());

                        println!("[{}]: {}", pid, line.join(" = "));
                        line.clear();
                        live_process.remove(&pid);
                        if live_process.len() == 0 { break }
                        else { continue }
                    },
                    wait::WaitStatus::PtraceEvent(pid, signal, c) => {
                        info!{"[{}] Process {} by {:?}", pid, DEBUG_PTRACE_EVENT[c as usize], signal};
                        pid
                    },
                    wait::WaitStatus::PtraceSyscall(pid) => {
                        // info!("[{}] Syscall", pid);
                        if live_process.get(&pid).is_none() {
                            live_process.insert(pid);
                        }
                        if proc_hook.get(&pid).is_none() {
                            proc_hook.insert(pid, (true, Vec::<String>::new()));
                        }
                        let regs = util::get_regs(pid);
                        let (prehook, line) = proc_hook.get_mut(&pid).unwrap();
                        if *prehook {
                            line.push(trace_prehook(&regs, pid));
                        } else {
                            line.push(trace_posthook(&regs));
                        
                            println!("[{}]: {}", pid, line.join(" = "));
                            line.clear();
                        }
                        *prehook = !*prehook;
                        // proc_hook.insert(pid, prehook);
                        pid
                    },
                    wait::WaitStatus::Signaled(pid, signal, b) => {
                        let core_dumped = if b { "(core dumped)" } else { "" };
                        info!("[{}] Signal {:?} received {}", pid, signal, core_dumped);
                        break
                    },
                    wait::WaitStatus::Stopped(pid, signal) => {
                        info!("[{}] Process stopped by {:?}", pid, signal);
                        pid
                        // break
                    },
                    wait::WaitStatus::Continued(pid) => {
                        info!("[{}] Process continued", pid);
                        break
                    },
                    wait::WaitStatus::StillAlive => {
                        info!("Still alive");
                        break
                    },
                };
                ptrace::syscall(actual_pid)?;
            }
        },
        ForkResult::Child => {
            ptrace::traceme()?;
            signal::raise(signal::Signal::SIGSTOP)?;
            execvp(prog, args)?;
        },
    }
    Ok(())
}
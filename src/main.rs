use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult};
use std::ffi::CString;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    loop {
        print!("rsh: ");
        io::stdout().flush()?;

        let mut buf = String::new();
        stdin.read_line(&mut buf).ok().expect("read_line failed");
        buf.pop();

        let cmd = parse(buf);
        exec(cmd);
    }
}

#[derive(Debug)]
struct Command {
    pub argc: usize,
    pub argv: Vec<String>,
}

fn parse(line: String) -> Command {
    let v: Vec<String> = line.split(' ').map(|c| c.to_string()).collect();
    Command {
        argc: v.len(),
        argv: v,
    }
}

fn exec(cmd: Command) {
    println!("{:?}", cmd);

    let args: Vec<CString> = cmd
        .argv
        .into_iter()
        .map(|v| CString::new(v).unwrap())
        .collect();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            execvp(&args[0], &args).unwrap();
        }
        Err(e) => {
            panic!("resessh panic!: {}", e);
        }
    }
}

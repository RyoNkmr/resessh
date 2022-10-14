use nix::unistd;
use std::ffi::CString;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    print!("resessh :> ");
    io::stdout().flush()?;

    for line in io::stdin().lines() {
        let cmd = parse(line.unwrap());
        exec(cmd);
    }
    Ok(())
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
    let args: Vec<CString> = cmd
        .argv
        .into_iter()
        .map(|v| CString::new(v).unwrap())
        .collect();
    unistd::execvp(&args[0], &args).unwrap();
}

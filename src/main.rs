use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).take(1).collect();
    if let [database_path] = &args[..] {
        let database = fs::File::open(database_path)?;
        let reader = BufReader::new(database);
        let cmd: HashMap<String, String> = reader
            .lines()
            .map(|entry| {
                entry
                    .unwrap()
                    .split(',')
                    .take(2)
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>()
            })
            .map(|mut v| (v.swap_remove(0), v.swap_remove(0)))
            .collect();
        let mut rofi = Command::new("rofi")
            .args(&["-dmenu", "-i", "-p", "Util"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("error on string rofi");
        rofi.stdin.as_mut().unwrap().write_all(
            cmd.keys()
                .map(|s| &**s)
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        )?;
        let selection = rofi.wait_with_output()?;
        if selection.status.success() {
            let output = String::from_utf8(selection.stdout).unwrap();
            let to_exec = cmd.get(output.trim()).expect("invalid selection");
            Command::new("bash").arg("-c").arg(to_exec).status()?;
            Ok(())
        } else {
            Err(io::ErrorKind::InvalidInput.into())
        }
    } else {
        Err(io::ErrorKind::InvalidInput.into())
    }
}

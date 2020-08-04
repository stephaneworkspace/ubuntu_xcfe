use std::io::{self, Write};
use std::process;
use std::process::Command;

fn main() {
    if !cfg!(target_os = "windows") {
        println!("Incompatible target os, need Microsoft Windows 10");
        process::exit(0);    }
    let output = {
        Command::new("x410.exe")
            .args(&["/desktop"])
            .output()
            .expect("failed to execute prcess x410.exe")
    };
    println!("status: {} | start /B x410.exe /desktop", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    let exec = r#"
"if [ -z \"$(pidof xfce4-session)\" ]; then export DISPLAY=$(cat /etc/resolv.conf | grep nameserver | awk '{print $2; exit;}'):0.0; xfce4-session; pkill '(gpg|ssh)-agent'; fi;"
        "#;
    let output = {
        Command::new("ubuntu2004.exe")
            .args(&["run", exec])
            .output()
            .expect("failed to execute prcess ubuntu2004.exe")
    };
    println!("status: {} | ubuntu2004.exe ru", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

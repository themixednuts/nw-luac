mod args;
use args::run;
use std::{
    fs::OpenOptions,
    io::{Read, Seek, Write},
    process::Command,
};

fn main() {
    let args = run();

    if args.luac.as_os_str().is_empty() {
        eprintln!("No luac5.1.exe path provided or found");
        std::process::exit(1);
    }

    let command = Command::new(args.luac.into_os_string())
        .args([
            "-s",
            "-o",
            &args.output.to_string_lossy(),
            &args.input.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run luac command");

    println!("{}", String::from_utf8_lossy(&command.stdout));

    if !command.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&command.stdout));
        eprintln!("{}", String::from_utf8_lossy(&command.stderr));
        std::process::exit(1);
    }

    let output = args.output;
    let mut temp = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(false)
        .open(&output)
        .expect("Failed to open luac file");

    let mut buffer = Vec::new();
    temp.read_to_end(&mut buffer)
        .expect("Luac file opened, but failed to read it");

    let mut prefix_buffer = vec![0x04, 0x00];
    prefix_buffer.extend_from_slice(&buffer);

    temp.seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek the start of Luac file");
    temp.write_all(&prefix_buffer)
        .expect("Failed to write to Luac");
}

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("bspc")
        .args(["subscribe", "report"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start bspc");

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut first = true;

        print!("[");
        let data = &line[1..]; // buang 'W'

        for part in data.split(':') {
            let bytes = part.as_bytes();
            if bytes.len() >= 2
                && b"fFoOuU".contains(&bytes[0])
                && bytes[1].is_ascii_digit()
            {
                if !first {
                    print!(",");
                }
                first = false;

                print!(
                    "{{\"id\":\"{}\",\"status\":\"{}\"}}",
                    &part[1..],
                    part.chars().next().unwrap()
                );
            }
        }

        println!("]");
        std::io::stdout().flush().unwrap();
    }
}

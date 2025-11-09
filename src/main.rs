use std::{
    io::{BufReader, Read, Write},
    process::{Command, Stdio},
};

use tempfile::NamedTempFile;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let width = args[1].parse::<usize>().expect("Failed to parse width");
    let frametrate = args[2].parse::<usize>().expect("failed to parse framerate");

    let config = format!(
        r#"
[general]
framerate = {}
bars = {}
[output]
method = raw
data_format = binary
bit_format = 8bit
channels = mono
waveform = 0"#,
        frametrate, width
    );

    let mut config_file = NamedTempFile::new().unwrap();
    config_file.write_all(config.as_bytes()).unwrap();

    let mut output = Command::new("cava")
        .args(["-p", config_file.path().to_str().unwrap()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("cava failed for some reason");

    let stdout = output.stdout.take().expect("Failed to get stdout handle");
    let mut reader = BufReader::new(stdout);

    loop {
        let mut buffer = vec![0_u8; width];
        match reader.read_exact(&mut buffer) {
            Ok(()) => create_blocks(buffer),
            Err(_) => continue,
        }
    }
}

fn create_blocks(bytes: Vec<u8>) {
    let block = |b: u8| -> char {
        match b {
            0..32 => '▁',
            32..64 => '▂',
            64..96 => '▃',
            96..128 => '▄',
            128..160 => '▅',
            160..192 => '▆',
            192..224 => '▇',
            224..=255 => '█',
        }
    };
    let res = bytes.into_iter().map(block);
    println!("{}", res.collect::<String>())
}

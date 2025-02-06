use std::fs::{File, OpenOptions};
use std::io::{BufWriter,BufReader};
use std::{
    env, fs,
    io::{stdin, stdout, Read, Write},
    process,
};

#[cfg(feature = "tui")]
pub mod tui;

use rb64::decode;
use rb64::Base64Encoder;
use rb64::Result;

fn transfer(from: &mut dyn Read, to: &mut dyn Write) -> std::io::Result<()> {
    let mut buf = [0_u8; 2048];
    while let Ok(n) = from.read(&mut buf) {
        if n == 0 { break }
        to.write_all(&buf[..n])?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let conf = Config::parse(env::args().skip(1)).unwrap();

    match conf.operation() {
        Operation::Encode => {
            for filename in conf.files() {
                let file = BufReader::new(File::open(filename)?);
                let mut encoder = Base64Encoder::new(file);
                if conf.files().len() > 1 {
                    let filename = filename.to_owned() + ".base64";
                    let mut out = BufWriter::new(OpenOptions::new().create(true).truncate(true).open(filename)?);
                    transfer(&mut encoder, &mut out)?;
                } else {
                    transfer(&mut encoder, &mut stdout().lock())?;
                }
            }
            if conf.files().is_empty() {
                let mut encoder = Base64Encoder::new(stdin().lock());
                transfer(&mut encoder, &mut stdout().lock())?;
            }
        }
        Operation::Decode => {
            for file in conf.files() {
                let data = fs::read_to_string(file)?;
                let dec = decode(&data).unwrap_or_else(|err| {
                    println!("ERROR: {err}");
                    process::exit(1);
                });
                if conf.files().len() > 1 {
                    let file = file.to_owned() + ".decoded";
                    fs::write(&file, dec)?;
                } else {
                    stdout().write_all(&dec)?;
                }
            }
            if conf.files().is_empty() {
                let mut data = String::new();
                stdin().read_to_string(&mut data)?;
                let dec = decode(&data).unwrap_or_else(|err| {
                    println!("ERROR: {err}");
                    process::exit(1);
                });
                stdout().write_all(&dec).unwrap();
            }
        }
        #[cfg(feature = "tui")]
        Operation::Tui => {
            return tui::tui_run();
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
pub enum Operation {
    Encode,
    Decode,
    #[cfg(feature = "tui")]
    Tui,
}

pub struct Config {
    operation: Operation,
    files: Vec<String>,
}

impl Config {
    pub fn parse(args: impl Iterator<Item = String>) -> Result<Self> {
        let mut conf = Self::default();
        for arg in args {
            match arg.as_str() {
                "-e" => conf.operation = Operation::Encode,
                "-d" => conf.operation = Operation::Decode,
                #[cfg(feature = "tui")]
                "-tui" => conf.operation = Operation::Tui,
                "-h" | "--help" => help(),
                _ => conf.files.push(arg),
            }
        }
        Ok(conf)
    }
    pub fn operation(&self) -> Operation {
        self.operation
    }
    pub fn files(&self) -> &[String] {
        &self.files
    }
}

fn help() -> ! {
    println!(
        "\
RB64: Base 64 encoder and decoder.
USAGE: rb64 [-e | -d] [files...]
OPTIONS:
    -e   Encode
    -d   Decode

If no files are given, reads stdin and outputs to stdout"
    );
    std::process::exit(0);
}

impl Default for Config {
    fn default() -> Self {
        Self {
            #[cfg(feature = "tui")]
            operation: Operation::Tui,
            #[cfg(not(feature = "tui"))]
            operation: Operation::Encode,
            files: Vec::new(),
        }
    }
}

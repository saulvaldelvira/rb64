use std::fs::{File, OpenOptions};
use std::io::{BufWriter,BufReader};
use std::{
    env, fs,
    io::{stdin, stdout, Read, Write},
    process,
};

#[cfg(feature = "gui")]
pub mod gui;

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

    match conf.operation {
        Operation::Encode => {
            for filename in &conf.files {
                let file = BufReader::new(File::open(filename)?);
                let mut encoder = Base64Encoder::new(file);
                if conf.files.len() > 1 {
                    let filename = filename.to_owned() + ".base64";
                    let out = OpenOptions::new()
                                          .create(true)
                                          .truncate(true)
                                          .open(filename)?;
                    let mut out = BufWriter::new(out);
                    transfer(&mut encoder, &mut out)?;
                } else {
                    transfer(&mut encoder, &mut stdout().lock())?;
                }
            }
            if conf.files.is_empty() {
                let mut encoder = Base64Encoder::new(stdin().lock());
                transfer(&mut encoder, &mut stdout().lock())?;
            }
        }
        Operation::Decode => {
            for file in &conf.files {
                let data = fs::read_to_string(file)?;
                let dec = decode(&data).unwrap_or_die(1);
                if conf.files.len() > 1 {
                    let file = file.to_owned() + ".decoded";
                    fs::write(&file, dec)?;
                } else {
                    stdout().write_all(&dec)?;
                }
            }
            if conf.files.is_empty() {
                let mut data = String::new();
                stdin().read_to_string(&mut data)?;
                let dec = decode(&data).unwrap_or_die(1);
                stdout().write_all(&dec).unwrap();
            }
        },
        #[cfg(feature = "gui")]
        Operation::Gui => {
            gui::start_gui().unwrap_or_die(2);
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
pub enum Operation {
    Encode,
    Decode,
    #[cfg(feature = "gui")]
    Gui,
}

pub struct Config {
    operation: Operation,
    files: Vec<String>,
}

impl Config {
    pub fn parse<It, S>(args: It) -> Result<Self>
    where
        S: AsRef<str> + Into<String>,
        It: IntoIterator<Item = S>,
    {
        let mut conf = Self::default();
        for arg in args.into_iter() {
            match arg.as_ref() {
                "-e" => conf.operation = Operation::Encode,
                "-d" => conf.operation = Operation::Decode,
                #[cfg(feature = "gui")]
                "-gui" => conf.operation = Operation::Gui,
                "-h" | "--help" => help(),
                _ => conf.files.push(arg.into()),
            }
        }
        Ok(conf)
    }
}

fn help() -> ! {
    #[cfg(not(feature = "gui"))]
    const SHORTFLAG: &str = "";
    #[cfg(not(feature = "gui"))]
    const DESCRIPTION: &str = "";

    #[cfg(feature = "gui")]
    const SHORTFLAG: &str = " | -gui";
    #[cfg(feature = "gui")]
    const DESCRIPTION: &str = "    -gui  Start GUI\n";

    println!("\
RB64: Base 64 encoder and decoder.
USAGE: rb64 [-e | -d{SHORTFLAG}] [files...]
OPTIONS:
    -e    Encode
    -d    Decode
{DESCRIPTION}
If no files are given, reads stdin and outputs to stdout"
);
    std::process::exit(0);
}

impl Default for Config {
    fn default() -> Self {
        Self {
            #[cfg(feature = "gui")]
            operation: Operation::Gui,
            #[cfg(not(feature = "gui"))]
            operation: Operation::Encode,
            files: Vec::new(),
        }
    }
}

trait UnwrapOrDie<T> {
    fn unwrap_or_die(self, ec: i32) -> T;
}

impl<T, E> UnwrapOrDie<T> for ::core::result::Result<T, E>
where
    E: ::core::fmt::Display
{
    fn unwrap_or_die(self, ec: i32) -> T {
        self.unwrap_or_else(|err| {
            eprintln!("ERROR: {err}");
            process::exit(ec);
        })
    }
}

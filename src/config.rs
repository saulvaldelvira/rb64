use rb64::Result;

#[derive(Clone, Copy)]
pub enum Operation {
    Encode, Decode,
    #[cfg(feature = "tui")]
    Tui
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
                _ => conf.files.push(arg),
            }
        }
        if !conf.files.is_empty() {
            conf.operation = Operation::Decode;
        }
        Ok(conf)
    }
    pub fn operation(&self) -> Operation { self.operation }
    pub fn files(&self) -> &[String] { &self.files }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            #[cfg(feature = "tui")]
            operation: Operation::Tui,
            #[cfg(not(feature = "tui"))]
            operation: Operation::Decode,
            files: Vec::new(),
        }
    }
}

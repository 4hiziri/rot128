extern crate getopts;
use getopts::Options;
use std::env;
use std::io::{Read, Write};

trait Converter {
    fn new(key: u8) -> Self;
    fn convert(&self, ch: u8) -> u8;

    fn convert_slice(&self, text: &[u8]) -> Vec<u8> {
        text.iter().map(|x| self.convert(x.to_be())).collect()
    }
}

struct Encoder {
    key: u8,
}

struct Decoder {
    key: u8,
}

impl Converter for Encoder {
    fn new(key: u8) -> Encoder {
        Encoder { key: key }
    }

    fn convert(&self, ch: u8) -> u8 {
        let max = u8::max_value() as u16 + 1;
        let ech: u16 = ch as u16 + self.key as u16;

        (ech % max) as u8
    }
}

impl Converter for Decoder {
    fn new(key: u8) -> Decoder {
        Decoder { key: key }
    }

    fn convert(&self, ch: u8) -> u8 {
        if ch < self.key {
            let max = u8::max_value();
            max - (self.key - ch - 1)
        } else {
            ch - self.key
        }
    }
}

fn parse_args(args: &[String]) -> getopts::Matches {
    let mut opts = Options::new();
    opts.optflag("d", "decode", "decode input");
    opts.optflag("e", "encode", "encode input");
    opts.optopt("k", "key", "key for encoding (or decoding): <0-255>", "key");

    match opts.parse(args) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    }
}

fn get_key(num_str_opt: Option<String>) -> Result<u8, std::num::ParseIntError> {
    match num_str_opt {
        Some(k) => k.parse::<u8>(),
        None => Ok(128),
    }
}

fn convert<T: Converter>(converter: T) {
    let stdin = std::io::stdin();
    let in_lock = stdin.lock();
    let mut reader = std::io::BufReader::new(in_lock);

    let stdout = std::io::stdout();
    let out_lock = stdout.lock();
    let mut writer = std::io::BufWriter::new(out_lock);

    let mut buf: [u8; 1] = [0; 1];

    while let Ok(_) = reader.read_exact(&mut buf) {
        writer.write(&[converter.convert(buf[0])]).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let matches = parse_args(&args[1..]);

    let key = match get_key(matches.opt_str("k")) {
        Ok(k) => k,
        Err(_) => panic!("Invalid key"),
    };

    if matches.opt_present("d") {
        convert(Decoder::new(key));
    } else {
        convert(Encoder::new(key));
    };
}

use std::io;
use std::fs;

fn encode_byte(ch: u8, shift: u8) -> u8 {
    let max = u8::max_value() as u16 + 1;
    let ech: u16 = ch as u16 + shift as u16;

    (ech % max) as u8
}

#[allow(dead_code)]
fn decode_byte(ch: u8, shift: u8) -> u8 {
    if ch < shift {
        let max = u8::max_value();
        max - (shift - ch - 1)
    } else {
        ch - shift
    }
}

fn encode(text: &[u8]) -> Vec<u8> {
    text.iter().map(|x| encode_byte(x.to_be(), 128)).collect()
}

#[allow(dead_code)]
fn decode(text: &[u8]) -> Vec<u8> {
    text.iter().map(|x| decode_byte(x.to_be(), 128)).collect()
}

fn main() {
    let mut buf: Vec<_> = vec![];
    {
        use io::Read;

        let mut fin = io::BufReader::new(fs::File::open("test.txt").unwrap());
        fin.read_to_end(&mut buf).unwrap();
    }

    let cipher = encode(&buf);

    {
        use io::Write;

        let mut fout = io::BufWriter::new(fs::File::create("test.txt").unwrap());
        fout.write_all(&cipher).unwrap();

    }
}

#[test]
fn test_encode_byte() {
    assert_eq!(encode_byte(0, 14), 14);
    assert_eq!(encode_byte(255, 1), 0);
    assert_eq!(encode_byte(254, 23), 21);
}

#[test]
fn test_decode_byte() {
    assert_eq!(decode_byte(encode_byte(0, 14), 14), 0);
    assert_eq!(decode_byte(encode_byte(255, 1), 1), 255);
    assert_eq!(decode_byte(encode_byte(254, 23), 23), 254);
}

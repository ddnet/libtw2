use arrayvec::ArrayVec;
use std::ascii;
use std::fmt;
use std::mem;
use std::ops;
use std::str;

pub struct AlmostString([u8]);

impl AlmostString {
    pub fn new(bytes: &[u8]) -> &AlmostString {
        unsafe {
            mem::transmute(bytes)
        }
    }
}

pub struct Bytes([u8]);

impl Bytes {
    pub fn new(bytes: &[u8]) -> &Bytes {
        unsafe {
            mem::transmute(bytes)
        }
    }
}

struct Byte {
    string: ArrayVec<[u8; 4]>,
}

impl Byte {
    fn new(byte: u8) -> Byte {
        let mut string = ArrayVec::new();
        if byte == b'\\' || byte == b'\"' {
            string.push(b'\\');
            string.push(byte);
        } else {
            string.extend(ascii::escape_default(byte));
        }
        Byte {
            string: string,
        }
    }
}

impl ops::Deref for Byte {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(&self.string)
        }
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(f.write_str("b\""));
        for &byte in &self.0 {
            try!(f.write_str(&Byte::new(byte)));
        }
        try!(f.write_str("\""));
        Ok(())
    }
}

impl fmt::Debug for AlmostString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: Replace this with a UTF-8 decoder.
        let string = String::from_utf8_lossy(&self.0);
        fmt::Debug::fmt(&string, f)
    }
}

impl fmt::Display for AlmostString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: Replace this with a UTF-8 decoder.
        let string = String::from_utf8_lossy(&self.0);
        if string.chars().any(|c| c < ' ' || c == '"') {
            fmt::Debug::fmt(&string, f)
        } else {
            fmt::Display::fmt(&string, f)
        }
    }
}

use std::io;

fn read_or_panic<R: io::Read>(stream: &mut R, bytes: &mut [u8]) {
    match stream.read_exact(bytes) {
        Ok(()) => (),
        Err(error) => panic!("Cannot read from stream: {error}"),
    };
}

pub fn read_u8<R: io::Read>(stream: &mut R) -> u8 {
    let mut bytes = [0u8; 1];
    read_or_panic(stream, &mut bytes);
    bytes[0]
}

pub fn read_i8<R: io::Read>(stream: &mut R) -> i8 {
    let mut bytes = [0u8; 1];
    read_or_panic(stream, &mut bytes);
    i8::from_le_bytes(bytes)
}

pub fn read_u16<R: io::Read>(stream: &mut R) -> u16 {
    let mut bytes = [0u8; 2];
    read_or_panic(stream, &mut bytes);
    u16::from_le_bytes(bytes)
}

pub fn read_i16<R: io::Read>(stream: &mut R) -> i16 {
    let mut bytes = [0u8; 2];
    read_or_panic(stream, &mut bytes);
    i16::from_le_bytes(bytes)
}

pub fn read_u32<R: io::Read>(stream: &mut R) -> u32 {
    let mut bytes = [0u8; 4];
    read_or_panic(stream, &mut bytes);
    u32::from_le_bytes(bytes)
}

pub fn read_i32<R: io::Read>(stream: &mut R) -> i32 {
    let mut bytes = [0u8; 4];
    read_or_panic(stream, &mut bytes);
    i32::from_le_bytes(bytes)
}

pub fn read_u64<R: io::Read>(stream: &mut R) -> u64 {
    let mut bytes = [0u8; 8];
    read_or_panic(stream, &mut bytes);
    u64::from_le_bytes(bytes)
}

pub fn read_i64<R: io::Read>(stream: &mut R) -> i64 {
    let mut bytes = [0u8; 8];
    read_or_panic(stream, &mut bytes);
    i64::from_le_bytes(bytes)
}

pub fn read_f32<R: io::Read>(stream: &mut R) -> f32 {
    let mut bytes = [0u8; 4];
    read_or_panic(stream, &mut bytes);
    f32::from_le_bytes(bytes)
}

pub fn read_f64<R: io::Read>(stream: &mut R) -> f64 {
    let mut bytes = [0u8; 8];
    read_or_panic(stream, &mut bytes);
    f64::from_le_bytes(bytes)
}

/// For reference see:
/// https://winprotocoldoc.blob.core.windows.net/productionwindowsarchives/MS-NRBF/%5bMS-NRBF%5d.pdf#%5B%7B%22num%22%3A66%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C69%2C670%2C0%5D
pub fn read_variable_length<R: io::Read>(stream: &mut R) -> usize {
    let mut length = 0usize;
    let mut num_bytes = 0;
    loop {
        let byte = read_u8(stream);
        length += ((byte & 0b01111111) as usize) << (num_bytes * 7);
        num_bytes += 1;
        if (byte & 0b10000000) == 0 {
            return length;
        }
    }
}

pub fn read_lps<R: io::Read>(stream: &mut R) -> String {
    let length = read_variable_length(stream);
    let mut data = vec![0u8; length];
    read_or_panic(stream, data.as_mut_slice());
    match String::from_utf8(data) {
        Ok(string) => string,
        Err(err) => panic!("Failed to decode UTF8 data: {err}"),
    }
}

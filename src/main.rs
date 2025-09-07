use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // 1. Get the filename from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // 2. Open the file
    let mut file = File::open(filename)?;

    // 3. Read the file in chunks
    let mut buffer = [0u8; 16]; // 16 bytes per line
    let mut offset: usize = 0;

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // EOF
        }

        // 4. Print offset
        print!("{:08X}: ", offset);

        // 5. Print bytes in hex
        for i in 0..bytes_read {
            print!("{:02X} ", buffer[i]);
        }

        // Add padding if last line is short
        if bytes_read < buffer.len() {
            for _ in bytes_read..buffer.len() {
                print!("   ");
            }
        }

        // 6. Print ASCII representation
        print!("|");
        for i in 0..bytes_read {
            let c = buffer[i];
            if c.is_ascii_graphic() || c == b' ' {
                print!("{}", c as char);
            } else {
                print!(".");
            }
        }
        println!("|");

        offset += bytes_read;
    }

    Ok(())
}

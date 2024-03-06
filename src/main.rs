use std::{io::{self, Read, Write}, sync::Mutex};

use clap::Parser;
use crossterm::terminal;

#[derive(Parser, Debug)]
struct Args {
    baud_rate: u32,
    #[arg()]
    device: String,
}


fn main() {
    let args = Args::parse();
    let sp = Mutex::new(serialport::new(args.device, args.baud_rate).open().unwrap());
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    terminal::enable_raw_mode().unwrap();

    std::thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut buf = [0; 128];
                let count = stdin.read(&mut buf).unwrap();
                let data = &buf[..count];

                let _ = sp.lock().unwrap().write(data).unwrap();
            }
        });

        loop {
            let mut buf = [0; 128];
            let count = sp.lock().unwrap().read(&mut buf).unwrap();
            let data = &buf[..count];

            let _ = stdout.write(data).unwrap();
        }
    });
}

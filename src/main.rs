use std::{io::{self, Read, Write}, sync::Mutex};

use clap::Parser;
use crossterm::terminal;
use serial2::SerialPort;

#[derive(Parser, Debug)]
struct Args {
    baud_rate: u32,
    #[arg()]
    device: String,
}


fn main() {
    let args = Args::parse();
    let sp = Mutex::new(SerialPort::open(args.device, args.baud_rate).unwrap());
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    terminal::enable_raw_mode().unwrap();

    std::thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut buf = [0; 1];
                let count = stdin.read(&mut buf).unwrap();
                let data = &buf[..count];

                eprintln!("Data: {:?}", data);
                let _ = sp.lock().unwrap().write(data).unwrap();
                sp.lock().unwrap().flush().unwrap();
            }
        });

        loop {
            let mut buf = [0; 1];
            let count = match sp.lock().unwrap().read(&mut buf) {
                    Ok(count) => count,
                    Err(err) => match err.kind() {
                        io::ErrorKind::TimedOut => continue,
                        _ => panic!("{:?}", err),
                    },
                };

            let data = &buf[..count];

            let _ = stdout.write(data).unwrap();
            stdout.flush().unwrap();
        }
    });
}

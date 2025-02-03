use std::{env, net::TcpStream};
use std::io::{BufReader, Write};
use openssl::ssl::{SslConnector, SslMethod};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("Usage: net_demo host port");
    } else {
        let stdin = std::io::stdin();
        let mut buffer = String::new();
        println!("Enter your message below. Type 'FINISHED' when done.");
        loop {
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            if input == "FINISHED" {
                break;
            } else {
                buffer.push_str(input.as_str());
                buffer.push('\n');
            }
        }

        let host = args[1].as_str();
        let port = args[2].as_str();
        let header = format!("{host}:{port}");
        let tcp = TcpStream::connect(header).unwrap();
        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let mut stream = connector.connect(host, tcp).unwrap();
        write!(stream, "{buffer}").unwrap();

        let mut reader = BufReader::new(stream);
        
    }
}
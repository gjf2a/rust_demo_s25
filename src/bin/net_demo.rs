use std::sync::Arc;
use std::{env, net::TcpStream};
use std::io::{BufReader, Read, Write};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("Usage: net_demo host port");
    } else {
        // Obtain standard set of trusted certificates
        let root_store = rustls::RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.into()
        };

        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let stdin = std::io::stdin();
        let mut buffer = String::new();
        println!("Enter your message below. Type 'FINISHED' when done.");
        loop {
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            if input.trim() == "FINISHED" {
                break;
            } else {
                buffer.push_str(input.as_str());
            }
        }

        let server_name = args[1].clone().try_into().unwrap();
        let host = args[1].as_str();
        let port = args[2].as_str();
        let header = format!("{host}:{port}");
        let mut tcp = TcpStream::connect(header).unwrap();
        let mut connector = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
        let mut stream = rustls::Stream::new(&mut connector, &mut tcp);
        write!(stream, "{buffer}").unwrap();

        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader.read_to_string(&mut response).unwrap();
        println!("{response}");
    }
}
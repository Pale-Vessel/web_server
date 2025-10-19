use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to get stream");

        println!("Connection established!");
    }
}
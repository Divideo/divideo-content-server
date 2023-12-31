mod models;
mod database;

use std::net::{TcpListener, TcpStream};

fn handle_connection(stream: TcpStream) {}

fn main() -> std::io::Result<()> {
    // Listen to http requests
    // Return Title and Thumbnail

    let listener = TcpListener::bind("0.0.0.0:5050")?;

    for connection in listener.incoming() {
        let Ok(connection) = connection else {
            continue;
        };

        std::thread::spawn(move || handle_connection(connection));
    }

    Ok(())
}

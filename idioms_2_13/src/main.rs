// Easy do initialization

use std::net::TcpStream;

struct Connection {
    name: String,
    stream: TcpStream,
}

impl Connection {
    /// Sends a request over the connection.
    /// 
    /// # Example
    /// ```
    /// # fn call_send(connection: Connection) {
    /// let _response = connection.send_request();
    /// }
    /// ```
    fn send_request(&self) {

    }
}

fn main() {
    println!("Hello, world!");
}

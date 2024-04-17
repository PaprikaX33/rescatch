//! The module that handles the server and their TCP socket behaviour
mod traits;
use crate::error;
pub use traits::TCPHandler;

/// Ip configuration for the tcp server
pub struct Ipconf {
    /** The listening IP address of the server
    # Note
    Use ip "0.0.0.0" to listen to any ip
    # Future note
    Might be better to use a dedicated ip structure or tuple containing the 4 u8 vars.
     */
    pub ip: String,

    ///The port where the server listens
    pub port: u32,
}
impl Ipconf {
    fn to_bind(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

/// Starts the server
pub fn start_server<F>(conf: Ipconf, handler: F) -> std::io::Result<u8>
where
    F: TCPHandler,
{
    let listener = std::net::TcpListener::bind(conf.to_bind().as_str()).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        match handler.execute(stream) {
            Ok(val) => {
                if val == 100 {
                    return Ok(0);
                }
            }
            Err(x) => {
                println!("{:?}", x);
            }
        }
    }
    Ok(0) // Always return 0, otherwise there is an error in binding or listening.
}

mod traits;
use crate::error;
pub use traits::TCPHandler;
pub struct Ipconf {
    pub ip: String,
    pub port: u32,
}
pub fn start_server<F>(conf: Ipconf, handler: F) -> std::io::Result<u8>
where
    F: TCPHandler,
{
    let listener =
        std::net::TcpListener::bind(format!("{}:{}", conf.ip, conf.port).as_str()).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        match handler.execute(stream) {
            Ok(_) => (),
            Err(x) => {
                println!("{:?}", x);
            }
        }
    }
    Ok(0)
}

use crate::error;
pub struct Ipconf {
    pub ip: String,
    pub port: u32,
}
pub fn start_server<F>(conf: Ipconf, handler: F) -> std::io::Result<u8>
where
    F: Fn(std::net::TcpStream) -> Result<u8, error::ServerHandlerError>,
{
    let listener =
        std::net::TcpListener::bind(format!("{}:{}", conf.ip, conf.port).as_str()).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        match handler(stream) {
            Ok(_) => (),
            Err(x) => {
                println!("{:?}", x);
            }
        }
    }
    Ok(0)
}

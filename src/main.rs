//use std::net::TcpListener;
mod error;
mod html;
mod response;
//mod http_parser;
mod parser;
mod tcpio;
use parser::FromBuf;
use std::io::Write;

fn main() -> std::io::Result<std::process::ExitCode> {
    let mut config: tcpio::Ipconf = tcpio::Ipconf {
        ip: "0.0.0.0".to_string(),
        port: 7999,
    };
    let mut itr = std::env::args();
    let _ = itr.next();
    while let Some(args) = itr.next() {
        match args.as_str() {
            "-h" | "--help" => {
                println!("./program [OPTIONS] [port]");
                println!("   -h -? --help : help");
                println!("  Port defaults to 7999");
                return Ok(std::process::ExitCode::from(0));
            }
            argm => {
                config.port = match argm.parse() {
                    Ok(x) => x,
                    Err(x) => {
                        println!("{:?}", x);
                        return Ok(std::process::ExitCode::from(20));
                    }
                }
            }
        }
    }
    println!("Start listening for connection in port {}", config.port);
    Ok(std::process::ExitCode::from(tcpio::start_server(
        config,
        Handler {},
    )?))
}

struct Handler {}
impl tcpio::TCPHandler for Handler {
    type Err = <parser::HttpRequestMessage as FromBuf>::Err;
    fn execute(&self, mut stream: std::net::TcpStream) -> Result<u8, Self::Err> {
        println!("New connection");
        let mut buf_reader = std::io::BufReader::new(&mut stream);
        println!("Request received");
        let Ok(request) = parser::HttpRequestMessage::from_buf(&mut buf_reader) else {
            let response = "HTTP/1.0 400 Bad Request\r\n\r\n<html><head><title>BadReq</title></head><body>Bad Request</body></html>";
            stream.write_all(response.as_bytes()).unwrap();
            drop(stream);
            return Ok(0);
        };
        println!("{}", request);
        let response = format!(
            "HTTP/1.0 200 OK\r\n\r\n<html><head><title>Local</title></head><body>{}</body></html>",
            html::sanitize(format!("{}", request).as_str())
        );
        stream.write_all(response.as_bytes()).unwrap();
        drop(stream);
        Ok(0)
    }
}

//use std::net::TcpListener;
mod error;
//mod http_parser;
mod parser;
mod tcpio;
use parser::FromBuf;

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
        connection_handler,
    )?))
}

fn connection_handler(mut stream: std::net::TcpStream) -> Result<u8, error::ServerHandlerError> {
    println!("New connection");
    let mut buf_reader = std::io::BufReader::new(&mut stream);
    println!("Request received");
    let request = parser::HttpRequestMessage::from_buf(&mut buf_reader)?;
    println!("{}", request);
    drop(stream);
    Ok(0)
}

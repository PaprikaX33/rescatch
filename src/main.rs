//use std::net::TcpListener;
mod error;
mod http_parser;
mod tcpio;
use std::io::BufRead;
use std::io::{Read, Write};
use std::str::FromStr;

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
    println!("Starting listeing for connection in port {}", config.port);
    Ok(std::process::ExitCode::from(tcpio::start_server(
        config,
        connection_handler,
    )?))
}

fn connection_handler(mut stream: std::net::TcpStream) -> Result<u8, error::ServerHandlerError> {
    let mut buf_reader = std::io::BufReader::new(&mut stream);
    /*let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        if http_request[0] == "GET / HTTP/1.0" {
            let status_line = "HTTP/1.1 200 OK";
            let contents = "<html><head><title>Testing</title></head><body>Hello There</body></html>";
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        } else if http_request[0] == "GET / HTTP/1.1" {
            let status_line = "HTTP/1.0 200 OK";
            let contents = "<html><head><title>Testing</title></head><body>Hello There. Downgrade to 1.0 please</body></html>";
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
            stream.write_all(response.as_bytes()).unwrap();
    }*/
    let mut strrep = String::new();
    buf_reader.read_to_string(&mut strrep)?;
    let header = http_parser::HttpRequestHeader::from_str(strrep.as_str())?;
    println!("{}", header);
    drop(stream);
    //println!("Request: {:#?}", http_request);
    Ok(0)
}

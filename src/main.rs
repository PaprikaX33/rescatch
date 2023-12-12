//use std::net::TcpListener;
use std::io::BufRead;
struct Cmd {
    port: u32,
}

fn main() -> std::process::ExitCode {
    let mut config: Cmd = Cmd { port: 7999 };
    let mut itr = std::env::args();
    let _ = itr.next();
    while let Some(args) = itr.next() {
        match args.as_str() {
            "-h" | "--help" => {
                println!("./program [OPTIONS] [port]");
                println!("   -h -? --help : help");
                println!("  Port defaults to 7999");
                return std::process::ExitCode::from(0);
            }
            argm => {
                config.port = match argm.parse() {
                    Ok(x) => x,
                    Err(x) => {
                        println!("{:?}", x);
                        return std::process::ExitCode::from(20);
                    }
                }
            }
        }
    }
    println!("Starting listeing for connection in port {}", config.port);
    std::process::ExitCode::from(start_server(config))
}

fn start_server(conf: Cmd) -> u8 {
    let listener =
        std::net::TcpListener::bind(format!("127.0.0.1:{}", conf.port).as_str()).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        match connection_handler(stream) {
            Ok(_) => (),
            Err(x) => {
                println!("{:?}", x);
            }
        }
    }
    0
}

fn connection_handler(mut stream: std::net::TcpStream) -> Result<(), std::io::Error> {
    let buf_reader = std::io::BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    Ok(())
}

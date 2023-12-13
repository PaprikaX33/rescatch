//use std::net::TcpListener;
mod tcpio;

fn main() -> std::process::ExitCode {
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
    std::process::ExitCode::from(tcpio::start_server(config))
}

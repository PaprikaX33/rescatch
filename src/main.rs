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
    println!("{}", config.port);
    std::process::ExitCode::from(0)
}

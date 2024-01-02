pub trait TCPHandler {
    type Err: std::fmt::Debug + std::fmt::Display;
    fn execute(&self, stream: std::net::TcpStream) -> Result<u8, Self::Err>;
}

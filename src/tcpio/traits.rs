//! Traits declaration for the tcpio module

/** Traits for the TCP handler
TCPHandler is the trait that defines the callback function for the behaviour of the server
*/
pub trait TCPHandler {
    /** Requirement for the type of Err
    It is recommended to set a concrete type for Err
     */
    type Err: std::fmt::Debug + std::fmt::Display;
    fn execute(&self, stream: std::net::TcpStream) -> Result<u8, Self::Err>;
}

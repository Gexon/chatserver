/// A stateful wrapper around a non-blocking stream. This connection is not
/// the SERVER connection. This connection represents the client connections
/// _accepted_ by the SERVER connection.
pub struct Connection {
    // handle to the accepted socket
    sock: TcpStream,

    // token used to register with the poller
    pub token: Token,

    // set of events we are interested in
    interest: Ready,

    // messages waiting to be sent out
    send_queue: Vec<Rc<Vec<u8>>>,

    // track whether a connection needs to be (re)registered
    is_idle: bool,

    // track whether a connection is reset
    is_reset: bool,

    // track whether a read received `WouldBlock` and store the number of
    // byte we are supposed to read
    read_continuation: Option<u64>,

    // track whether a write received `WouldBlock`
    write_continuation: bool,
}
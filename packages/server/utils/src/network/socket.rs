use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};

pub enum ListenerError {
    BindError,
    LocalAddrError,
}

pub fn create_listener(ip: IpAddr, port: Option<u16>) -> Result<TcpListener, ListenerError> {
    let port = port.unwrap_or(0);
    let listener = match TcpListener::bind((ip, port)) {
        Err(_) => return Err(ListenerError::BindError),
        Ok(listener) => listener,
    };
    let local_addr = match listener.local_addr() {
        Err(_) => return Err(ListenerError::LocalAddrError),
        Ok(addr) => addr,
    };
    println!("Listener: {}:{}", local_addr.ip(), local_addr.port());
    Ok(listener)
}

pub enum SocketError {
    AcceptError,
    SetOptionError,
}

pub fn get_socket(listener: TcpListener) -> Result<(TcpStream, SocketAddr), SocketError> {
    println!("Waiting for connection...");
    let (stream, addr) = match listener.accept() {
        Err(_) => return Err(SocketError::AcceptError),
        Ok(socket) => socket,
    };
    if stream.set_nodelay(true).is_err() {
        return Err(SocketError::SetOptionError);
    }
    println!("Accepted: {}:{}", addr.ip(), addr.port());
    Ok((stream, addr))
}

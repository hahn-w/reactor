use reactor::{Reactor, TcpServer, Protocol, ProtocolFactory, Transport};
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

struct EchoProtocol {
}

impl Protocol for EchoProtocol {
    
    fn make_connection(&mut self, _: Transport) { todo!() }
    fn data_received(&mut self, _: &[u8]) { todo!() }
}

struct EchoFactory { }

impl ProtocolFactory for EchoFactory {
    
    fn build_protocol(&self) -> Box<dyn Protocol> { 
        Box::new(EchoProtocol {})
    }
}

impl EchoFactory {
    fn new() -> Box<dyn ProtocolFactory> {
        Box::new(EchoFactory {})
    }
}


fn main() {

    let mut reactor = Reactor::new();
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddr::V4(SocketAddrV4::new(ip, 9000));
    let factory = EchoFactory::new();
    TcpServer::new(socket, &mut reactor, factory);

    reactor.run();
}

use reactor::{Protocol, Transport};

struct EchoServer {
}

impl Protocol for EchoServer {
    fn make_connection(&mut self, transport: Transport) {

    }

    fn data_received(&mut self, data: &[u8]) {
    }
}

fn main() {
}

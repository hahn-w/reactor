use reactor::{create_listen, accept, Event, Events, PollSelector};
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

fn connected(e: &Event) {
    let fd = accept();
}

fn main() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddr::V4(SocketAddrV4::new(ip, 9000));

    let fd = create_listen(socket);

    let mut selector = PollSelector::new();
    selector.add_event(
        Event {
            fd,
            handle: connected
        },
        Events::READ, 0);

    loop {
        selector.process_events(1000);
    }
}

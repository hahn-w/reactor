use reactor::{Transport, Protocol};

struct Echo {
    transport: Option<Transport>,
    executed: bool,
}

impl Protocol for Echo {
    fn make_connection(&mut self, transport: Transport) {
        self.transport = Some(transport);
    }

    fn data_received(&mut self, data: &[u8]) {
        self.executed  = true;
        match self.transport.as_mut() {
            Some(transport) => {
                transport.write(data);
            }
            None => ()
        };
    }
}

#[test]
fn can_run() {    
    let mut protocol = Echo {
        transport: None,
        executed: false,
    };
    protocol.data_received(&[1 as u8]);

    assert_eq!(true, protocol.executed);
}
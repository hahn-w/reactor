use crate::transport::Transport;

pub trait Protocol {
    fn make_connection(&mut self, transport: Transport);
    
    fn data_received(&mut self, data: &[u8]);
}

pub trait ProtocolFactory {
    fn build_protocol(&self) -> Box<dyn Protocol>;
}
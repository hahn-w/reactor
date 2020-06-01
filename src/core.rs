use crate::descriptor::{ReadDescriptor, DescriptorCollect, FileDescriptor};
use crate::selector::{Selector, PollSelector};
use crate::event::{Event};
use std::convert::AsMut;


pub type add_event = fn(Event);

pub struct Reactor {
    pub selector: Box<dyn Selector>,
}

impl Reactor {
    pub fn run(&mut self) {
        
        // let s: &mut Self = &mut &self;
        loop {
            // self.selector.poll();
        }
        
    }

    pub fn new() -> Reactor {
        Reactor {
            selector: PollSelector::new()
        }
    }
}
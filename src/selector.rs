use libc::{poll, pollfd};
use crate::event::{Event, Events};
use std::collections::HashMap;

pub trait Selector {
    fn add_event(&mut self, event: Event, events: Events, flags: u32);
    fn process_events(&mut self, timeout: i32);
}

pub struct PollSelector {
    events: HashMap<i32, Event>,
    fd_set: Vec<pollfd>,
    nevents: usize,
}

impl Selector for PollSelector {

    fn add_event(&mut self, event: Event, events: Events, _flags: u32) {
        unsafe { self.fd_set.set_len(self.nevents + 1) };

        self.fd_set[self.nevents] =
            pollfd {
                fd: event.fd,
                events: match events {
                    Events::READ => libc::POLLIN,
                    Events::WRITE => libc::POLLOUT
                },
                revents: 0
            }
        ;
        self.nevents += 1;
        self.events.insert(event.fd, event);
    }

    fn process_events(&mut self, timeout: i32) {
        unsafe {
            let ready = poll(self.fd_set.as_mut_ptr(), self.nevents as u32, timeout);

            for i in 0..ready {
                match self.events.get(&i) {
                    Some(event) => {
                        let callback = (* event).handle;
                        callback(event);
                    }
                    None => {
                    }
                }
            }
        }
    }
}

impl PollSelector {
    pub fn new() -> Box<dyn Selector> {
        Box::new(PollSelector {
            // TODO: 像 malloc 一样申请地址？
            events: HashMap::new(),
            fd_set: Vec::with_capacity(1024),
            nevents: 0
        })
    }
}
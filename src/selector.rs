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

    fn add_event(&mut self, event: Event, events: Events, flags: u32) {
        self.events.insert(event.fd, event);

        self.fd_set.push(
            pollfd {
                fd: event.fd,
                events: match events {
                    Events::READ => libc::POLLIN,
                    Events::WRITE => libc::POLLOUT
                },
                revents: 0
            }
        );
        self.nevents += 1;
    }

    fn process_events(&mut self, timeout: i32) {
        unsafe {
            let ready = poll(self.fd_set.as_mut_ptr(), self.fd_set.len() as u32, timeout);

            for i in 0..ready {
                let fd = self.fd_set[i as usize].fd;
                let event = self.events.get(&i).unwrap();
                let handle = (* event).handle;
                handle(* event);
            }
        }
    }
}

impl PollSelector {
    pub fn new() -> PollSelector {
        PollSelector {
            // TODO: 像 malloc 一样申请地址？
            events: HashMap::new(),
            fd_set: Vec::with_capacity(1024),
            nevents: 0
        }
    }
}
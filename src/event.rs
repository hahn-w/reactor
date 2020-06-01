pub type EventHandle = fn(&Event);

pub struct Event {
    pub fd: i32,
    pub handle: EventHandle,
}

pub enum Events {
    READ = -1,
    WRITE = -2
}
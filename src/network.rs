use std::net::{SocketAddr};

pub fn create_listen(address: SocketAddr) -> i32 {

    unsafe {
        use libc::{socket, bind, listen, AF_INET, AF_INET6, SOCK_STREAM, socklen_t};

        let family = match address {
            SocketAddr::V4(..) => AF_INET,
            SocketAddr::V6(..) => AF_INET6
        };

        let fd = socket(family, SOCK_STREAM, 0);

        let (addr, len) = match address {
            SocketAddr::V4(ref a) => {
                (a as *const _ as *const _, std::mem::size_of_val(a) as socklen_t)
            }
            SocketAddr::V6(ref a) => {
                (a as *const _ as *const _, std::mem::size_of_val(a) as socklen_t)
            }
        };

        bind(fd, addr, len as _);
        listen(fd, 128);

        libc::ioctl(fd, libc::FIONBIO, &1);

        fd
    }
}

pub fn accept(fd: i32) -> i32 {
    unsafe {
        let mut storage: libc::sockaddr_un = unsafe { std::mem::zeroed() };
        let mut len = std::mem::size_of_val(&storage) as libc::socklen_t;
        libc::accept(fd, &mut storage as *mut _ as *mut _, &mut len)
    }
}
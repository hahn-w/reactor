# ReactorIO

reactorio 是基于rust的事件驱动的网络开发库。MIT协议。支持Mac、Windows、和Linux。正在开发中。

reactorio初衷是提供一个中文社区的网络开发库。如果正在进行日常的网络应用开发、学习TCP/IP的编程开发，请联系我进群交流。

Hahn发起，联系方式：

- hahn_w@163.com
- 微信 `imjuimju`
- 微博 `hahn2020`

```rust
fn connected(event: &Event) {
    
}

fn main() {
    let fd = networking::listen((127, 0, 0, 1), 8000);
    let event = reactory.fd_event_new(fd, &connected, EventType.READ);
    reactor.add_event(event);
    reactory.run();
}
```
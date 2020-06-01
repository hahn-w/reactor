use reactor::Reactor;

#[test]
fn test_reactor() {
    let mut reactor = Reactor::new();
    reactor.run();
}
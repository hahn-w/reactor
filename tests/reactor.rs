use reactor::Reactor;

#[test]
fn test_reactor() {
    let reactor = Reactor::new();
    reactor.run();
}
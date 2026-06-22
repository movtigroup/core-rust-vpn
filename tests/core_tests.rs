use std::net::TcpListener;

#[test]
fn test_local_port_binding() {
    let addr = "127.0.0.1:20001";
    let listener = TcpListener::bind(addr).expect("Should bind once");

    let second_listener = TcpListener::bind(addr);
    assert!(second_listener.is_err(), "Should fail to bind twice to same port");

    drop(listener);
    let third_listener = TcpListener::bind(addr);
    assert!(third_listener.is_ok(), "Should bind again after drop");
}

#[tokio::test]
async fn test_async_port_binding() {
    let addr = "127.0.0.1:20002";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Should bind once");
    drop(listener);
}

#[macro_export]
macro_rules! run {
    () => {
        use $crate::server::server::Server;
        Server::start().await;
    };
}

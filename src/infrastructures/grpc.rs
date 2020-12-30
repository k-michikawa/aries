use tonic::transport;

pub fn get_server() -> transport::server::Server {
    transport::Server::builder()
}

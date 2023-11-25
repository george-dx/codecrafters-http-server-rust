use http_server::http_server::HttpServer;

mod http_server;

const IP_ADDR: &str = "127.0.0.1";
const PORT: u16 = 4221;
fn main() {
    let http_server = HttpServer::new(IP_ADDR.to_string(), PORT);
    http_server.listen();
}

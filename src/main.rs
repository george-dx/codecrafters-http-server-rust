use http_server::http_server::HttpServer;

mod http_server;

fn main() {
    let http_server = HttpServer::new(String::from("127.0.0.1"), 4221);
    http_server.listen();
}

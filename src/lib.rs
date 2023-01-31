use actix_web::dev::Server;
use actix_web::{App, HttpServer};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new()).bind("127.0.0.1:8000")?.run();
    Ok(server)
}

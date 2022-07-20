use rich_strong_lib::{log::Log, config::Config, head::LineType, server::Server};

fn main() {
    Log::create_log_dir();
    let app_addr = Config::init();
    let mut app = Server::new(app_addr,LineType::Operator);
    app.start();
}

mod api;
mod repository;

use api::{
    actuator::version,
    actuator::health_check,
    file::delete_file,
    file::get_file,
    file::post_file,
    file::put_file,
};

use repository::config::load_config;

use actix_web::{App, web, HttpServer, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config().unwrap();
    
    env_logger::init();

    // Create the data directory if it does not exist
    std::fs::create_dir_all(&config.data_path)?;

    // Create the Actix state
    let app_state = web::Data::new(config.clone());

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(app_state.clone())
            .wrap(logger)
            .service(health_check)
            .service(version)
            .service(delete_file)
            .service(get_file)
            .service(post_file)
            .service(put_file)
    })
    .workers(config.worker_count)
    .bind(("127.0.0.1", config.server_port)).expect("Failed to bind to address")
    .run()
    .await
}
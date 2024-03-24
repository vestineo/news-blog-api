mod api;
mod models;
mod repository;

use actix_files as fs;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::post_api::{
    create_post, get_post, get_posts, hello, posts_by_author, posts_by_category, search_posts,
};
use env_logger::Env;
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(hello)
            .service(create_post)
            .service(get_post)
            .service(get_posts)
            .service(search_posts)
            .service(posts_by_category)
            .service(posts_by_author)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

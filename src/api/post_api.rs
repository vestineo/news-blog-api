use crate::{
    models::post_model::{JsonPost, Post},
    repository::mongodb_repo::MongoRepo,
};
use actix_web::{
    get, post,
    web::{Data, Json, Path, Query},
    HttpRequest, HttpResponse, Responder,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PostRange {
    page: u64,
    limit: i64,
}

#[derive(Serialize)]
struct PostsRespose {
    posts: Vec<Post>,
    total: u64,
}

#[get("/")]
async fn hello() -> impl Responder {
    std::println!("hotiing");
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[post("/post")]
pub async fn create_post(db: Data<MongoRepo>, new_post: Json<JsonPost>) -> HttpResponse {
    let post_detail = db.create_post(Post::from(new_post.into_inner())).await;
    match post_detail {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/post/{id}")]
pub async fn get_post(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let post_detail = db.get_post(&id).await;
    match post_detail {
        Ok(post) => {
            return HttpResponse::Ok().json(post);
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/posts")]
pub async fn get_posts(db: Data<MongoRepo>, range: Query<PostRange>) -> HttpResponse {
    let posts = db.get_posts(range.page, range.limit).await;
    let total = db.get_total_posts().await;
    match posts {
        Ok(post) => match total {
            Ok(total_count) => {
                return HttpResponse::Ok().json(PostsRespose {
                    posts: post,
                    total: total_count,
                })
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/category/{query}")]
pub async fn posts_by_category(
    db: Data<MongoRepo>,
    req: HttpRequest,
    range: Query<PostRange>,
) -> HttpResponse {
    let category: String = req.match_info().query("query").parse().unwrap();
    let posts = db
        .posts_by_category(category.clone(), range.page, range.limit)
        .await;

    let total = db.get_total_posts_category(category.clone()).await;
    match posts {
        Ok(post) => match total {
            Ok(total_count) => {
                return HttpResponse::Ok().json(PostsRespose {
                    posts: post,
                    total: total_count,
                })
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/author/{query}")]
pub async fn posts_by_author(
    db: Data<MongoRepo>,
    req: HttpRequest,
    range: Query<PostRange>,
) -> HttpResponse {
    let author: String = req.match_info().query("query").parse().unwrap();
    let posts = db
        .posts_by_author(author.clone(), range.page, range.limit)
        .await;
    let total = db.get_total_posts_author(author.clone()).await;
    match posts {
        Ok(post) => match total {
            Ok(total_count) => {
                return HttpResponse::Ok().json(PostsRespose {
                    posts: post,
                    total: total_count,
                })
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/search/{query}")]
pub async fn search_posts(db: Data<MongoRepo>, req: HttpRequest) -> HttpResponse {
    let query: String = req.match_info().query("query").parse().unwrap();
    let posts = db.search_posts(query).await;
    match posts {
        Ok(post) => return HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

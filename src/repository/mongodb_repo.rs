use std::env;
extern crate dotenv;
use dotenv::dotenv;
use futures::TryStreamExt;

use crate::models::post_model::Post;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    options::{CountOptions, FindOptions},
    results::InsertOneResult,
    Client, Collection,
};

pub struct MongoRepo {
    col: Collection<Post>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("news");
        let col: Collection<Post> = db.collection("Post");
        MongoRepo { col }
    }

    pub async fn create_post(&self, new_post: Post) -> Result<InsertOneResult, Error> {
        let new_doc = Post {
            id: None,
            title: new_post.title,
            date: new_post.date,
            author: new_post.author,
            category: new_post.category,
            content: new_post.content,
            image: new_post.image,
        };
        let post = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating post");
        Ok(post)
    }

    pub async fn get_post(&self, id: &String) -> Result<Post, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let post_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(post_detail.unwrap())
    }

    pub async fn get_total_posts(&self) -> Result<u64, Error> {
        let count_options = CountOptions::builder().build();
        let count = self
            .col
            .count_documents(None, count_options)
            .await
            .ok()
            .expect("total count fetch failed");
        Ok(count)
    }

    pub async fn get_total_posts_category(&self, category: String) -> Result<u64, Error> {
        let query = format!("^{}.*", category);
        let filter = doc! {"category": {"$regex": query, "$options": "i" }};

        let count_options = CountOptions::builder().build();
        let count = self
            .col
            .count_documents(Some(filter), count_options)
            .await
            .ok()
            .expect("total count fetch failed");
        Ok(count)
    }

    pub async fn get_total_posts_author(&self, author: String) -> Result<u64, Error> {
        let query = format!("^{}.*", author);
        let filter = doc! {"author": {"$regex": query, "$options": "i" }};
        let count_options = CountOptions::builder().build();
        let count = self
            .col
            .count_documents(Some(filter), count_options)
            .await
            .ok()
            .expect("total count fetch failed");
        Ok(count)
    }

    pub async fn get_posts(&self, page: u64, limit: i64) -> Result<Vec<Post>, Error> {
        let skip = (page - 1) * limit as u64;
        let find_options = FindOptions::builder()
            .sort(doc! {"date": -1})
            .skip(skip)
            .limit(limit)
            .build();
        let mut cursors = self
            .col
            .find(None, find_options)
            .await
            .ok()
            .expect("Error getting list of posts");
        let mut posts: Vec<Post> = Vec::new();
        while let Some(post) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            posts.push(post)
        }
        Ok(posts)
    }

    pub async fn posts_by_category(
        &self,
        category: String,
        page: u64,
        limit: i64,
    ) -> Result<Vec<Post>, Error> {
        let skip = (page - 1) * limit as u64;
        let query = format!("^{}.*", category);
        let filter = doc! {"category": {"$regex": query, "$options": "i" }};
        let find_options = FindOptions::builder()
            .sort(doc! {"date": -1})
            .skip(skip)
            .limit(limit)
            .build();
        let mut cursors = self
            .col
            .find(Some(filter), find_options)
            .await
            .ok()
            .expect("Error getting list of posts");
        let mut posts: Vec<Post> = Vec::new();
        while let Some(post) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            posts.push(post);
        }
        Ok(posts)
    }

    pub async fn posts_by_author(
        &self,
        author: String,
        page: u64,
        limit: i64,
    ) -> Result<Vec<Post>, Error> {
        let skip = (page - 1) * limit as u64;
        let query = format!("^{}.*", author);
        let filter = doc! {"author": {"$regex": query, "$options": "i" }};
        let find_options = FindOptions::builder()
            .sort(doc! {"date": -1})
            .skip(skip)
            .limit(limit)
            .build();
        let mut cursors = self
            .col
            .find(Some(filter), find_options)
            .await
            .ok()
            .expect("Error getting list of posts");
        let mut posts: Vec<Post> = Vec::new();
        while let Some(post) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            posts.push(post);
        }
        Ok(posts)
    }

    pub async fn search_posts(&self, search: String) -> Result<Vec<Post>, Error> {
        let filter = doc! { "$text": { "$search": search.clone() } };
        let sort = doc! { "score": { "$meta": "textScore" } };
        let opts = FindOptions::builder().sort(sort).build();

        let cursors = self
            .col
            .find(filter, opts)
            .await
            .ok()
            .expect("Error getting list of posts");

        Ok(cursors
            .try_collect()
            .await
            .ok()
            .expect("Error getting list of posts"))
    }
}

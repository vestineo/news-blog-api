use mongodb::bson::{oid::ObjectId, serde_helpers::bson_datetime_as_rfc3339_string, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonPost {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub date: DateTime,
    pub author: String,
    pub category: String,
    pub content: String,
    #[serde(default)]
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub date: DateTime,
    pub author: String,
    pub category: String,
    pub content: String,
    #[serde(default)]
    pub image: String,
}

impl From<JsonPost> for Post {
    fn from(json_post: JsonPost) -> Post {
        Post {
            id: None,
            title: json_post.title,
            date: json_post.date,
            author: json_post.author,
            category: json_post.category,
            content: json_post.content,
            image: json_post.image,
        }
    }
}

impl From<Post> for JsonPost {
    fn from(post: Post) -> JsonPost {
        JsonPost {
            id: None,
            title: post.title,
            date: post.date,
            author: post.author,
            category: post.category,
            content: post.content,
            image: post.image,
        }
    }
}

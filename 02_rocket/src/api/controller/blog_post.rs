use derive_new::new;
use rocket::{serde::json::Json, get, post};
use serde::Serialize;
use serde::Deserialize;

// Actions
#[get("/")]
pub fn get_all() -> Json<Vec<BlogPost>> {
    Json(vec![ 
        BlogPost { id: 10, title: "".to_string(), body: "".to_string(), published: true },
        BlogPost { id: 11, title: "".to_string(), body: "".to_string(), published: true },
        BlogPost { id: 12, title: "".to_string(), body: "".to_string(), published: true },
        BlogPost { id: 13, title: "".to_string(), body: "".to_string(), published: true },
        BlogPost { id: 14, title: "".to_string(), body: "".to_string(), published: true },
        BlogPost { id: 15, title: "".to_string(), body: "".to_string(), published: true },
        BlogPost { id: 16, title: "".to_string(), body: "".to_string(), published: true },
    ])
}

#[get("/random")]
pub fn get_random() -> Json<BlogPost> {
    Json( 
        BlogPost { id: 10, title: "".to_string(), body: "".to_string(), published: true }
    )
}

#[post("/", data = "<blog_post>")]
pub fn create(blog_post: Json<BlogPost>) -> Json<i32> {

    Json(1)
}

// DTOs
#[derive(Serialize, Deserialize, new)]
pub struct BlogPost {
    #[new(default)]
    id: i32,
    #[new(value = "".to_owned())]
    pub title: String,
    #[new(value = "".to_owned())]
    pub body: String,
    #[new(value = "false")]
    pub published: bool,
}
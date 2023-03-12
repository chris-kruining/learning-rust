#[macro_use] extern crate rocket;
#[macro_use] extern crate derive_new;

use learning_rust::api::controller::blog_post;

#[get("/")]
fn index() -> &'static str {
    "hello je moeder!"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/blog-posts", routes![blog_post::get_all])
        .mount("/blog-posts", routes![blog_post::get_random])
        .mount("/blog-posts", routes![blog_post::create])
}
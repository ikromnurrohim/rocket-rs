#[macro_use] extern crate rocket;
mod tera;

use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;
use rocket::tokio::time::{sleep, Duration};


#[get("/")]
fn index() -> RawHtml<&'static str>{
    RawHtml(r#"See <a href="tera">Tera</a>."#)
}

#[get("/")]
fn test() -> &'static str {
    "My Name is Ikrom Nur Rohim !"
}

#[get("/saying/<name>")]
fn saying(name:&str) -> String{
    format!("Hello, {} !!!", name)
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello_cool(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You are cool {} year old {}", age, name)
    }else{
        format!("{}, We need talk about your coolnes", name)
    }
}


#[get("/foo/<_>/bar")]
fn foo_bar() -> &'static str{
    "Foo ___ bar"
}

#[get("/<_..>")]
fn everything()-> &'static str{
    "Hey, you're here."
}

#[get("/delay/<second>")]
async fn delay(second: u64) -> String{
    sleep(Duration::from_secs(second)).await;
    format!("await for a {} second", second)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/tera", routes![tera::index, tera::hello, tera::about])
    .register("/tera", catchers![tera::not_found])
    .attach(Template::custom(|engines| {
        tera::customize(&mut engines.tera);
    }))
    .mount("/test", routes![test, delay, saying, hello_cool, foo_bar, everything])
}

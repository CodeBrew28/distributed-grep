#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
fn main() {
  rocket::ignite()
    .mount("/", routes![index])
    .launch();
}

#[get("/")]
fn index() -> String {
  "Hello Wooooooorrrrlllld! Not much a blog yet, eh?".to_string()
}
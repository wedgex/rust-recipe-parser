#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use recipes::Recipe;
use rocket_contrib::json::Json;
use serde::Deserialize;

mod recipes;
mod spoon;

#[derive(Deserialize)]
struct ParseRequest {
  url: String,
}

#[post("/parse", data = "<body>", format = "json")]
fn parse(body: Json<ParseRequest>) -> Option<Json<Recipe>> {
  // TODO actual error responses
  match Recipe::from(body.url.clone()) {
    Ok(Some(recipe)) => Some(Json(recipe)),
    Ok(None) => None,
    Err(_) => None,
  }
}

fn main() {
  rocket::ignite().mount("/", routes![parse]).launch();
}

#![feature(proc_macro_hygiene, decl_macro)]

use std::collections::HashMap;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

// HOME PAGE
#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", context)
}

// EVENTS API
#[derive(Serialize, Deserialize, Debug)]
struct Event {
    id: Option<usize>,
    title: String,
}

#[get("/?<page>")]
fn get_events(page: Option<usize>) -> JsonValue {
    let page = if let Some(i) = page {
        i
    } else {
        1
    };

    json!({
        "events": [],
        "page": page,
    })
}

#[get("/<id>")]
fn get_event(id: usize) -> Json<Event> {
    Json(Event {
        id: Some(10),
        title: String::from("First Event"),
    })
}

#[delete("/<id>")]
fn delete_event(id: usize) -> status::Custom<JsonValue> {
    status::Custom(
        Status::NoContent,
        json!({
            "status": "ok",
            "id": id,
        })
    )
}

#[post("/", data = "<body>")]
fn new_event(body: Json<Event>) -> status::Created<Json<Event>> {
    status::Created(
        uri!("/events", get_event: 10).to_string(),
        Some(Json(
            Event {
                id: Some(10),
                title: String::from("First Event"),
            }
        ))
    )
}

#[put("/<id>", data = "<body>")]
fn update_event(id: usize, body: Json<Event>) -> Json<Event> {
    Json(Event {
        id: Some(id),
        title: String::from("First Event"),
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/events", routes![get_events, get_event, delete_event, new_event, update_event])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}

#![feature(proc_macro_hygiene, decl_macro)]

use std::collections::HashMap;
use std::sync::RwLock;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

// HOME PAGE
#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", context)
}

// EVENTS API
type EventsList = RwLock<HashMap<usize, Event>>;

#[derive(Serialize, Deserialize, Clone)]
struct Event {
    id: Option<usize>,
    title: String,
}

#[get("/?<page>")]
fn get_events(page: Option<usize>, events_list: State<EventsList>) -> JsonValue {
    let events_list = events_list.read().unwrap();
    let id = next_insert_id(events_list.keys().collect::<Vec<&usize>>());

    // TODO Implement paging when switching to database (hashmap is unordered and iterates in no particular order) 
    let page = if let Some(i) = page {
        i
    } else {
        1
    };

    json!({
        "events": *events_list,
        "page": page,
    })
}

#[get("/<id>")]
fn get_event(id: usize, events_list: State<EventsList>) -> Option<Json<Event>> {
    let events_list = events_list.read().unwrap();

    if events_list.contains_key(&id) {
        Some(Json(
            events_list.get(&id).unwrap().clone()
        ))
    } else {
        None
    }
}

#[delete("/<id>")]
fn delete_event(id: usize, events_list: State<EventsList>) -> Option<status::Custom<JsonValue>> {
    let events_list = &mut events_list.write().unwrap();

    if events_list.contains_key(&id) {
        events_list.remove_entry(&id);

        Some(status::Custom(
            Status::NoContent,
            json!({
                "status": "ok",
                "id": id,
            })
        ))
    } else {
        None
    }
}

#[post("/", data = "<body>")]
fn new_event(body: Json<Event>, events_list: State<EventsList>) -> status::Created<Json<Event>> {
    let events_list = &mut events_list.write().unwrap();

    let id = next_insert_id(events_list.keys().collect::<Vec<&usize>>());
    let mut event = body.into_inner();
    event.id = Some(id);

    events_list.insert(id, event.clone());

    status::Created(
        uri!("/events", get_event: id).to_string(),
        Some(Json(
            event
        ))
    )
}

fn next_insert_id(mut current_ids: Vec<&usize>) -> usize {
    current_ids.sort();

    current_ids.pop().unwrap() + 1
}

#[put("/<id>", data = "<body>")]
fn update_event(id: usize, body: Json<Event>) -> Json<Event> {
    Json(Event {
        id: Some(id),
        title: String::from("First Event"),
    })
}

fn rocket() -> rocket::Rocket {
    // Create mock database
    let mut events_hashmap = HashMap::new();
    events_hashmap.insert(1, Event { id: Some(1), title: String::from("First Event")});
    events_hashmap.insert(2, Event { id: Some(2), title: String::from("Second Event")});
    events_hashmap.insert(3, Event { id: Some(3), title: String::from("Third Event")});
    
    let events_list: EventsList = RwLock::new(events_hashmap);

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/events", routes![get_events, get_event, delete_event, new_event, update_event])
        .attach(Template::fairing())
        .manage(events_list)
}

fn main() {
    rocket().launch();
}

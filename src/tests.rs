use super::rocket;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn test_get_events() {
    let client = Client::new(rocket()).unwrap();
    let response = client.get("/events").dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_event() {
    let client = Client::new(rocket()).unwrap();
    let response = client.get("/events/10").dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_delete_event() {
    let client = Client::new(rocket()).unwrap();
    let response = client.delete("/events/10").dispatch();

    assert_eq!(response.status(), Status::NoContent);
}

#[test]
fn test_new_event() {
    let client = Client::new(rocket()).unwrap();
    let response = client.post("/events")
        .body(r#"{"id":10,"title":"Test Event"}"#)
        .dispatch()
    ;

    assert_eq!(response.status(), Status::Created);
}

#[test]
fn test_update_event() {
    let client = Client::new(rocket()).unwrap();
    let response = client.put("/events/10")
        .body(r#"{"id":10,"title":"Test Event"}"#)
        .dispatch()
    ;

    assert_eq!(response.status(), Status::Ok);
}

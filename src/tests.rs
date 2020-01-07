use super::rocket;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn test_get_events() {
    let client = Client::new(rocket()).unwrap();

    // Test default response
    let mut response = client.get("/events").dispatch();
    let mut expected_body = r#"{"events":{"1":{"id":1,"title":"First Event"},"2":{"id":2,"title":"Second Event"},"3":{"id":3,"title":"Third Event"}},"page":1}"#.to_string();
    
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(expected_body));

    // Test response for existing page
    // TODO This test currently only checks if the page parameter is correctly parsed, update expected_body when paging is implemented
    response = client.get("/events?page=2").dispatch();
    expected_body = r#"{"events":{"1":{"id":1,"title":"First Event"},"2":{"id":2,"title":"Second Event"},"3":{"id":3,"title":"Third Event"}},"page":2}"#.to_string();
    
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(expected_body));

    // Test response for non existing page
    // response = client.get("/events?page=3").dispatch();
    // expected_body = r#"{"events":{"1":{"id":1,"title":"First Event"},"2":{"id":2,"title":"Second Event"},"3":{"id":3,"title":"Third Event"}},"page":3}"#.to_string();
    
    // assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.body_string(), Some(expected_body));

    // Test response for invalid page value
    response = client.get("/events?page=test").dispatch();
    expected_body = r#"{"events":{"1":{"id":1,"title":"First Event"},"2":{"id":2,"title":"Second Event"},"3":{"id":3,"title":"Third Event"}},"page":1}"#.to_string();
    
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(expected_body));
}

#[test]
fn test_get_event() {
    let client = Client::new(rocket()).unwrap();

    // Test existing event response
    let mut response = client.get("/events/1").dispatch();
    let expected_body = r#"{"id":1,"title":"First Event"}"#.to_string();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(expected_body));

    // Test non existing event response
    response = client.get("/events/10").dispatch();

    assert_eq!(response.status(), Status::NotFound);

    // Test invalid event response
    response = client.get("/events/test").dispatch();

    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_delete_event() {
    let client = Client::new(rocket()).unwrap();
    
    // Test existing event delete
    let mut response = client.delete("/events/1").dispatch();

    assert_eq!(response.status(), Status::NoContent);

    // Test if the event is actually removed
    response = client.get("/events/1").dispatch();

    assert_eq!(response.status(), Status::NotFound);

    // Test non existing event delete
    response = client.get("/events/10").dispatch();

    assert_eq!(response.status(), Status::NotFound);

    // Test invalid event delete
    response = client.get("/events/test").dispatch();

    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_new_event() {
    let client = Client::new(rocket()).unwrap();

    // Test new event
    let response = client.post("/events")
        .body(r#"{"title":"Fourth Event"}"#)
        .dispatch()
    ;
    let expected_body = "/events/4";

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.headers().get_one("Location"), Some(expected_body));

    // Test if the event is actually created
    let location = response.headers().get_one("Location").unwrap();
    let mut response = client.get(location).dispatch();
    let expected_body = r#"{"id":4,"title":"Fourth Event"}"#.to_string();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(expected_body));
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

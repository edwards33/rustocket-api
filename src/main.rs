#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

mod person;
use person::{Person};

#[post("/", data = "<person>")]
fn create(mut person: Json<Person>) -> Json<Person> {
    person.id = Some(11);
    person
}

#[get("/")]
fn read() -> JsonValue {
    json!([
        "person 1", 
        "person 2"
    ])
}

#[put("/<id>", data = "<person>")]
fn update(id: i32, person: Json<Person>) -> Json<Person> {
    person
}

#[delete("/<id>")]
fn delete(id: i32) -> JsonValue {
    json!({"status": "ok"})
}

fn main() {
    rocket::ignite()
        .mount("/person", routes![create, update, delete])
        .mount("/persons", routes![read])
        .launch();
}

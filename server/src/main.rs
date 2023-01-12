#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, json::Json};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/flashcard_set/all")]
fn flashcard_sets_all() -> &'static str {
    "{all flashcards}"
}


// Just playing around with inputs, need to look into serialization
// if you want to test this out try:
// curl -X POST http://127.0.0.1:8000/flashcard_set -H "Content-Type: application/json" -d '{"title": "New Set"}'
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct FlashcardSet<'r> {
    title: &'r str
}
#[post("/flashcard_set", format="application/json", data = "<flashcard_set>")]
fn flashcard_set_new(flashcard_set: Json<FlashcardSet<'_>>) -> String {
    format!("You created a flashcard set with title: {}\n", flashcard_set.title)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, flashcard_sets_all, flashcard_set_new])
}
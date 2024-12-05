#[macro_use]
extern crate rocket;

use rocket::{fs::FileServer, serde::json::Json};
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

use ch24::generator::{find_all_solutions, generate_cardset};

#[derive(Serialize)]
struct Cards {
    cards: Vec<u64>,
    solutions: Vec<String>,
}

#[get("/cards", format = "json")]
fn generate_cards_route() -> Json<Cards> {
    let cardset = generate_cardset();
    let solutions = find_all_solutions(cardset.clone());

    let cards = Cards {
        cards: cardset
            .iter()
            .map(|c| c.eval().try_into().unwrap())
            .collect(),
        solutions: solutions
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<_>>(),
    };

    Json(cards)
}

#[get("/")]
fn index_route() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn run() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index_route, generate_cards_route])
        .mount("/static", FileServer::from("./static"))
}

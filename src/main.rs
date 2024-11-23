#[macro_use]
extern crate rocket;

use minijinja::{context, path_loader, Environment};
use rocket::{fs::FileServer, response::content::RawHtml, serde::json::Json, State};
use serde::Serialize;
mod generator;


#[derive(Serialize)]
struct Cards {
    cards: Vec<u64>,
    solutions: Vec<String>,
}

#[get("/cards", format = "json")]
fn generate_cards_route() -> Json<Cards> {
    let (cardset, solutions) = generator::generate_cardset();

    let cards = Cards {
        cards: cardset,
        solutions: solutions
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<_>>(),
    };

    Json(cards)
}

#[get("/")]
fn index_route(jinja_env: &State<Environment>) -> RawHtml<String> {
    RawHtml(
        jinja_env
            .get_template("index.html")
            .and_then(|t| t.render(context! {}))
            .unwrap(),
    )
}

#[launch]
fn run() -> _ {
    let jinja_env = {
        let mut env = Environment::new();
        env.set_loader(path_loader("templates"));
        env
    };

    rocket::build()
        .manage(jinja_env)
        .mount("/", routes![index_route, generate_cards_route])
        .mount("/static", FileServer::from("./static"))
}

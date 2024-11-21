#[macro_use]
extern crate rocket;

use minijinja::{context, path_loader, Environment};
use rocket::{response::content::RawHtml, State};

#[get("/hello")]
fn world(jinja_env: &State<Environment>) -> RawHtml<String> {
    RawHtml(
        jinja_env
            .get_template("index.html")
            .and_then(|t| t.render(context! {}))
            .unwrap()
    )
}

#[launch]
fn run() -> _ {
    let jinja_env = {
        let mut env = Environment::new();
        env.set_loader(path_loader("templates"));
        env
    };

    rocket::build().mount("/", routes![world]).manage(jinja_env)
}

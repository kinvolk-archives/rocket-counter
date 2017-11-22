#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

use rocket_contrib::Template;

static COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Serialize)]
struct TemplateContext {
    count: usize,
}

#[get("/")]
fn index() -> Template {
    COUNTER.fetch_add(1, Ordering::SeqCst);

    let ctx = TemplateContext{ count: COUNTER.load(Ordering::SeqCst) };

    Template::render("index", &ctx)
}

#[post("/")]
fn reset() {
    COUNTER.store(0, Ordering::SeqCst);
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/reset", routes![reset])
        .attach(Template::fairing())
        .launch();
}

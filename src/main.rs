#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

static COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;

#[get("/")]
fn index() -> String {
    COUNTER.fetch_add(1, Ordering::SeqCst);

    format!("Hello, world! {}", COUNTER.load(Ordering::SeqCst))
}

#[post("/")]
fn reset() {
    COUNTER.store(0, Ordering::SeqCst);
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/reset", routes![reset])
        .launch();
}

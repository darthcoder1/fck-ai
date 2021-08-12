#![feature(proc_macro_hygiene, decl_macro)]

use rocket::http::RawStr;
use rocket_contrib::serve::StaticFiles;

#[macro_use]
extern crate rocket;

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    let str = format!("Hello {}", name.as_str());

    str
}

fn main() {
    let static_files_path = concat!(env!("CARGO_MANIFEST_DIR"), "/pkg");
    println!("Path: {}", &static_files_path);
    rocket::ignite()
        .mount("/", StaticFiles::from(&static_files_path))
        .launch();
}

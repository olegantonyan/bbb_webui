#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;
use rocket::response::NamedFile;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[get("/")]
pub fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "Rust");
    Template::render("index", &context)
}

#[get("/<asset..>")]
fn assets(asset: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("/home/oleg/projects/bbb_webui/src/assets").join(asset)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, assets])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}

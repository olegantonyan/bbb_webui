#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::fairing::AdHoc;
use rocket::State;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;


struct AssetsDir(String);

#[get("/")]
pub fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "Rust");
    Template::render("index", &context)
}

#[get("/system")]
pub fn system() -> Template {
    let mut context = HashMap::new();
    let uptime_stdout = Command::new("uptime").output().expect("failed to execute `uptime`").stdout;
    let uptime = String::from_utf8(uptime_stdout).unwrap();
    context.insert("uptime", uptime);
    Template::render("system", &context)
}

#[get("/vpn")]
pub fn vpn() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "Rust");
    Template::render("vpn", &context)
}

#[post("/system/reboot")]
pub fn system_reboot() -> Template {
    let mut context = HashMap::new();
    context.insert("hello", "world");
    Command::new("reboot").spawn().expect("failed to execute `reboot`");
    Template::render("rebooting", &context)
}

#[get("/<asset..>")]
fn assets(asset: PathBuf, assets_dir: State<AssetsDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&assets_dir.0).join(asset)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, system, vpn, system_reboot, assets])
        .attach(Template::fairing())
        .attach(AdHoc::on_attach(|rocket| {
            let assets_dir = rocket.config().get_str("assets_dir").unwrap().to_string();
            Ok(rocket.manage(AssetsDir(assets_dir)))
        }))
}

fn main() {
    rocket().launch();
}

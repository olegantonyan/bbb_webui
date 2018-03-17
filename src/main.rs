#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

mod systemd;
mod models;

use rocket_contrib::Template;
use rocket::response::{NamedFile, Redirect};
use rocket::fairing::AdHoc;
use rocket::State;

use std::collections::HashMap;
use std::path::{Path, PathBuf};


struct AssetsDir(String);

#[get("/")]
pub fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("uptime", models::uptime::Uptime::default().execute());
    Template::render("index", &context)
}

#[get("/system")]
pub fn system() -> Template {
    let mut context = HashMap::new();
    context.insert("uptime", models::uptime::Uptime::default().execute());
    Template::render("system", &context)
}

#[get("/vpn")]
pub fn vpn() -> Template {
    let mut context = HashMap::new();

    let ovp = models::openvpn::OpenVPN::default();
    context.insert("status", ovp.status());
    context.insert("logs", ovp.logs());

    Template::render("vpn", &context)
}

#[post("/vpn/start")]
pub fn vpn_start() -> Redirect {
    let ovp = models::openvpn::OpenVPN::default();
    ovp.start();
    Redirect::to("/vpn")
}

#[post("/vpn/stop")]
pub fn vpn_stop() -> Redirect {
    let ovp = models::openvpn::OpenVPN::default();
    ovp.stop();
    Redirect::to("/vpn")
}

#[post("/vpn/restart")]
pub fn vpn_restart() -> Redirect {
    let ovp = models::openvpn::OpenVPN::default();
    ovp.restart();
    Redirect::to("/vpn")
}

#[post("/system/reboot")]
pub fn system_reboot() -> Template {
    let result = models::reboot::Reboot::default().execute();
    let mut context = HashMap::new();
    context.insert("message", result);
    Template::render("rebooting", &context)
}

#[get("/<asset..>")]
fn assets(asset: PathBuf, assets_dir: State<AssetsDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&assets_dir.0).join(asset)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, system, vpn, vpn_start, vpn_stop, vpn_restart, system_reboot, assets])
        .attach(Template::fairing())
        .attach(AdHoc::on_attach(|rocket| {
            let assets_dir = rocket.config().get_str("assets_dir").unwrap().to_string();
            Ok(rocket.manage(AssetsDir(assets_dir)))
        }))
}

fn main() {
    rocket().launch();
}

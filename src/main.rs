#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate libc;

mod initd;
mod models;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::fairing::AdHoc;
use rocket::State;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use initd::services::ServiceConfig;

struct AssetsDir(String);
pub struct InitD(initd::InitD);

#[get("/")]
pub fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[get("/system")]
pub fn system() -> Template {
    let mut context = HashMap::new();
    context.insert("uptime", models::uptime::Uptime::default().execute());
    Template::render("system", &context)
}

#[get("/vpn")]
pub fn vpn(initd: State<InitD>) -> Template {
    let mut context = HashMap::new();

    let service_name = initd::services::openvpn::OpenVPN::default().name();
    match initd.0.process(service_name) {
        Some(ref process) => {
            println!("{:?}", process.state());
            context.insert("pid", process.state().pid().unwrap_or(0));
        },
        None => {
            context.insert("pid", 0);
        }
    }

    Template::render("vpn", &context)
}

#[post("/system/reboot")]
pub fn system_reboot() -> Template {
    models::reboot::Reboot::default().execute();
    let context: HashMap<String, String> = HashMap::new();
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
        .attach(AdHoc::on_attach(|rocket| {
            Ok(rocket.manage(InitD(start_services())))
        }))
}

fn start_services() -> initd::InitD {
    let mut init = initd::InitD::default();
    let ovp = initd::services::openvpn::OpenVPN::default();
    init.start_service(&ovp);
    init
}

fn main() {
    //let mut init = initd::InitD::default();
    //let ovp = initd::services::openvpn::OpenVPN::default();
    //init.start_service(&ovp);
    //initd::Process::new(initd::openvpn::OpenVPN::default()).start();
    //let p = initd::process::Process::new();
    //p.start(ovp).join();
    //println!("p: {:?}", p.state());

    rocket().launch();
}

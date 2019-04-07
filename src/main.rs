#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

mod systemd;
mod rwfs;
mod models;

use rocket_contrib::templates::Template;
use rocket::response::{NamedFile, Redirect};
use rocket::fairing::AdHoc;
use rocket::State;
use rocket::request::{Form};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

struct AssetsDir(String);
pub struct VpnConfig(models::openvpn::Config);

#[derive(FromForm, Debug)]
pub struct VpnConfigfile {
    vpn_config: String
}

#[derive(FromForm, Debug)]
pub struct PoststartScript {
    script: String
}

#[get("/system")]
pub fn system() -> Template {
    let mut context = HashMap::new();
    context.insert("uptime", models::uptime::Uptime::default().execute());
    Template::render("system", &context)
}

#[get("/")]
pub fn vpn(vpn_config: State<VpnConfig>) -> Template {
    let mut context = HashMap::new();

    let ovp = models::openvpn::OpenVPN::new(&vpn_config.0);

    let status = match ovp.status() {
        Ok(data) => data,
        Err(data) => format!("Error: {}", data)
    };
    context.insert("status", vec![status]);

    let logs = match ovp.logs() {
        Ok(data) => data,
        Err(data) => format!("Error: {}", data)
    };
    context.insert("logs", vec![logs]);
    context.insert("available_configs", ovp.available_configs());
    context.insert("current_config", vec![ovp.current_config()]);
    context.insert("poststart", vec![ovp.poststart()]);

    Template::render("vpn", &context)
}

#[post("/vpn/start")]
pub fn vpn_start(vpn_config: State<VpnConfig>) -> Result<Redirect, Template> {
    let ovp = models::openvpn::OpenVPN::new(&vpn_config.0);
    match ovp.start() {
        Ok(_) => Ok(Redirect::to("/")),
        Err(data) => {
            let mut context = HashMap::new();
            context.insert("message", data);
            Err(Template::render("error", &context))
        }
    }
}

#[post("/vpn/stop")]
pub fn vpn_stop(vpn_config: State<VpnConfig>) -> Result<Redirect, Template> {
    let ovp = models::openvpn::OpenVPN::new(&vpn_config.0);
    match ovp.stop() {
        Ok(_) => Ok(Redirect::to("/")),
        Err(data) => {
            let mut context = HashMap::new();
            context.insert("message", data);
            Err(Template::render("error", &context))
        }
    }
}

#[post("/vpn/restart")]
pub fn vpn_restart(vpn_config: State<VpnConfig>) -> Result<Redirect, Template> {
    let ovp = models::openvpn::OpenVPN::new(&vpn_config.0);
    match ovp.restart() {
        Ok(_) => Ok(Redirect::to("/")),
        Err(data) => {
            let mut context = HashMap::new();
            context.insert("message", data);
            Err(Template::render("error", &context))
        }
    }
}

#[post("/vpn/change_config", data = "<configfile>")]
pub fn vpn_change_config(configfile: Form<VpnConfigfile>, vpn_config: State<VpnConfig>) -> Result<Redirect, Template> {
    let ovp = models::openvpn::OpenVPN::new(&vpn_config.0);
    match ovp.change_config(configfile.vpn_config.clone()) {
        Ok(_) => Ok(Redirect::to("/")),
        Err(data) => {
            let mut context = HashMap::new();
            context.insert("message", data);
            Err(Template::render("error", &context))
        }
    }
}

#[post("/vpn/poststart/save", data = "<script>")]
pub fn vpn_poststart_save(script: Form<PoststartScript>, vpn_config: State<VpnConfig>) -> Result<Redirect, Template> {
    let ovp = models::openvpn::OpenVPN::new(&vpn_config.0);

    match ovp.save_poststart(script.script.clone().replace("\r", "")) {
        Ok(_) => Ok(Redirect::to("/")),
        Err(data) => {
            let mut context = HashMap::new();
            context.insert("message", data);
            Err(Template::render("error", &context))
        }
    }
}

#[post("/system/reboot")]
pub fn system_reboot() -> Result<Redirect, Template> {
    match models::reboot::Reboot::default().execute() {
        Ok(_) => Ok(Redirect::to("/")),
        Err(data) => {
            let mut context = HashMap::new();
            context.insert("message", data);
            Err(Template::render("error", &context))
        }
    }
}

#[get("/<asset..>")]
fn assets(asset: PathBuf, assets_dir: State<AssetsDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&assets_dir.0).join(asset)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![system, vpn, assets, vpn_start, vpn_stop, vpn_restart, system_reboot, vpn_change_config, vpn_poststart_save])
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("assets", |rocket| {
            let assets_dir = rocket.config().get_str("assets_dir").unwrap().to_string();
            Ok(rocket.manage(AssetsDir(assets_dir)))
        }))
        .attach(AdHoc::on_attach("config file", |rocket| {
            let dir = rocket.config().get_str("vpn_config_dir").unwrap().to_owned();
            let fname = rocket.config().get_str("vpn_current_config_symlink_name").unwrap().to_owned();
            let srv = rocket.config().get_str("vpn_service_name").unwrap().to_owned();
            let suf = rocket.config().get_str("vpn_config_file_suffix").unwrap().to_owned();
            let ps = rocket.config().get_str("poststart_script").unwrap().to_owned();
            let cfg = models::openvpn::Config { dir: dir, current_config_symlink_name: fname, service_name: srv, vpn_config_file_suffix: suf, poststart_script: ps };
            Ok(rocket.manage(VpnConfig(cfg)))
        }))
}

fn main() {
    rocket().launch();
}

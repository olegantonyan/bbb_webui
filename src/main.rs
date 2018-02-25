extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate handlebars_iron;

use iron::prelude::*;
use router::Router;
use staticfile::Static;
use mount::Mount;
use handlebars_iron::{Template, HandlebarsEngine, DirectorySource};
use std::collections::HashMap;

const ASSETS_PATH: &'static str = "/home/oleg/projects/bbbui/src/assets/";

fn index(_: &mut Request) -> IronResult<Response> {
    //Ok(Response::with((iron::status::Ok, "Index")))
    let mut resp = Response::new();

    let mut data = HashMap::new();
    data.insert("name", "Rust");
    resp.set_mut(Template::new("index.html", data)).set_mut(iron::status::Ok);
    Ok(resp)
}

fn response_printer(_req: &mut Request, res: Response) -> IronResult<Response> {
    println!("Response produced: {}", res);
    Ok(res)
}

fn main() {
    let mut router = Router::new();
    router.get("/", index, "index");

    let mut mount = Mount::new();
    mount.mount("/static", Static::new(ASSETS_PATH));
    mount.mount("/", router);

    let mut chain = Chain::new(mount);
    chain.link_after(response_printer);


      let mut hbse = HandlebarsEngine::new();
      hbse.add(Box::new(DirectorySource::new(ASSETS_PATH, ".hbs")));
      if let Err(r) = hbse.reload() {
        panic!("{}", r);
      }

      chain.link_after(hbse);


    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}

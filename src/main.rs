#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;


mod routes {
    #[get("/")]
    pub fn index() -> &'static str {
        "Hello, world!"
    }
}

fn main() {
    rocket::ignite().mount("/", routes![routes::index]).launch();
}

//fn assets_path() -> &'static str {
//    if cfg!(debug_assertions) {
//        "./src/assets/"
//    } else {
//        "./assets/"
//    }
//}

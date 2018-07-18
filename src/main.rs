#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate postgres;

use postgres::{Connection, TlsMode};

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    let string = "postgres://postgres:alliesofphotoyodaarestrong@localhost/mydb";
    println!("{}",string);
    let conn = Connection::connect(string, TlsMode::None).unwrap();
    for row in &conn.query("SELECT * FROM webpage", &[&0]).unwrap() {
        let bar: String = row.get(0);
        println!("{}",bar)
    }
    NamedFile::open("static/index.html")

}




#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files])
}

fn main() {
    rocket().launch();
}

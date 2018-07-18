#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate mysql;

//use postgres::{Connection, TlsMode};
use rocket::State;
use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use mysql as my;



#[get("/")]
fn index(conn: State<String>) -> io::Result<NamedFile> {
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
    let string = "mysql://deadbeef:dlahfalsdfhieorpqwhfalkdsfhsadfwer@localhost:3306/mydb";
    //let string = "mysql://deadbeef:ldfakjlsdfwoiqtyweurynmc@localhost:3306/mydb";
    println!("{}",string);
    let pool = my::Pool::new(string).unwrap();
    rocket()
        .manage(pool)
        .launch();
}

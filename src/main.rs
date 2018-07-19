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
use mysql::from_row;



#[get("/")]
fn index(pool: State<my::Pool>) -> /*io::Result<NamedFile>*/ String {
    let mut val = vec!();
    for row in pool.inner().prep_exec("SELECT * from website", ()).unwrap() {
        let a: String = from_row(row.unwrap());
        println!("{}", a);
        val.push(a);
    }
//    NamedFile::open("static/index.html")
    val.get(0).unwrap().to_string()
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

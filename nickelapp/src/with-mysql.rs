#[macro_use]
extern crate nickel;
extern crate mysql;
extern crate nickel_mysql;

use nickel::{Nickel, HttpRouter};
use nickel_mysql::{MysqlMiddleware, MysqlRequestExtensions};
use mysql::value::from_value;

const DB_NAME: &'static str = "database";
const DB_USER: &'static str = "user";
const DB_PASS: &'static str = "pass";

struct Purin {
    id: u32,
    name: String,
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
       get "**" => |_req, _res| {
           "Hello world!"
       }
    });

    server.utilize(MysqlMiddleware::new(DB_NAME, DB_USER, DB_PASS));

    server.get("/test",
               middleware! { |request|
        let db_conn = request.db_connection();
        let players: Vec<Purin> = db_conn.prep_exec("SELECT id, name FROM purin", ()).map(|result| {
            result.map(|x| x.unwrap()).map(|mut row| {
                Purin {
                    id: from_value(row.take("id").unwrap()),
                    name: from_value(row.take("name").unwrap()),
                }
            }).collect()
        }).unwrap();

        for p in players.iter() {
            println!("id: {}, name: {}", p.id, p.name);
        }
    });

    server.listen("127.0.0.1:6767");
}

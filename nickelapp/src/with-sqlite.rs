#[macro_use]
extern crate nickel;
extern crate r2d2;
extern crate nickel_sqlite;
extern crate r2d2_sqlite;

use nickel::{Nickel, HttpRouter};
use nickel_sqlite::{SqliteMiddleware, SqliteRequestExtensions};

const DB_FILE: &'static str = "purin.sqlite";

const CRATE_TABLE: &'static str = "
CREATE TABLE purin (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
)";

struct Purin {
    id: i32,
    name: String,
}

fn main() {
    let mut server = Nickel::new();

    let mw = SqliteMiddleware::new(DB_FILE).expect("Unable to connect to database");
    let db = mw.pool.clone().get().unwrap();

    match db.execute(CRATE_TABLE, &[]) {
        Ok(_) => println!("create table"),
        Err(_) => {}
    };

    server.utilize(mw);

    server.utilize(router! {
       get "**" => |_req, _res| {
           "Hello world!"
       }
    });

    server.get("/test",
               middleware! { |request|
        let db_conn = request.db_conn().unwrap();
        let mut stmt = db_conn.prepare("SELECT id, name FROM purin").unwrap();
        let players = stmt.query_map(&[], |row| {
            Purin {
                id: row.get(0),
                name: row.get(1),
            }
        }).unwrap();

        for player in players {
            let p = player.unwrap();
            println!("id: {}, name: {}", p.id, p.name);
        }
    });

    server.listen("127.0.0.1:6767");
}

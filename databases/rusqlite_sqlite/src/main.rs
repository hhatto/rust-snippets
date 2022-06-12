extern crate rusqlite;

use rusqlite::Connection;

fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  age             INTEGER NOT NULL,
                  created         TEXT NOT NULL
                  )", []).unwrap();

    let name = "rustacean";
    let age = "20";
    conn.execute("INSERT INTO person (name, age, created)
                  VALUES (?1, ?2, datetime())",
                 &[&name, &age]).unwrap();

    let mut stmt = conn.prepare("SELECT id, name, age, created FROM person").unwrap();
    let mut cursor = stmt.query([]).expect("stmt.query() error");
    while let Some(r) = cursor.next().unwrap() {
        let id: i32 = r.get(0).unwrap();
        let name: String = r.get(1).unwrap();
        let age: i32 = r.get(2).unwrap();
        let created: String = r.get(3).unwrap();
        println!("{:?}, {}, {}, {}", id, name, age, created);
    }
}

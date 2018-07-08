extern crate rusqlite;

use rusqlite::Connection;

fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  age             INTEGER NOT NULL,
                  created         TEXT NOT NULL
                  )", &[]).unwrap();

    let name = "rustacean";
    let age = 20;
    conn.execute("INSERT INTO person (name, age, created)
                  VALUES (?1, ?2, datetime())",
                 &[&name, &age]).unwrap();

    let mut stmt = conn.prepare("SELECT id, name, age, created FROM person").unwrap();
    let mut cursor = stmt.query(&[]).expect("stmt.query() error");
    while let Some(row) = cursor.next() {
        let r = row.expect("result row error");
        let id: i32 = r.get(0);
        let name: String = r.get(1);
        let age: i32 = r.get(2);
        let created: String = r.get(3);
        println!("{:?}, {}, {}, {}", id, name, age, created);
    }
}

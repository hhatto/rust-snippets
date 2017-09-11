extern crate sqlite3;

use sqlite3::StatementUpdate;

fn main() {
    let mut conn = sqlite3::DatabaseConnection::in_memory().expect("sqlite open connection error.");

    conn.exec("CREATE TABLE person (
               id        INTEGER PRIMARY KEY,
               name      TEXT NOT NULL,
               age       INTEGER NOT NULL,
			   created   TEXT NOT NULL
              )").expect("create table error.");

    let name = "rustacean".to_owned();
    let age = 20;
    let mut cursor = conn.prepare(
        "INSERT INTO person (name, age, created) VALUES ($1, $2, datetime())"
        ).expect("prepare error");

    let r = cursor.update(&[&name, &age]).expect("cursor update() error");
    println!("change={}", r);

    let mut stmt = conn.prepare("SELECT * FROM person").expect("select prepare error");
    let mut results = stmt.execute();
    while let Ok(Some(row)) = results.step() {
        let id: i32 = row.column_int(0);
        let name: String = row.column_text(1).unwrap();
        let age: i32 = row.column_int(2);
        let created: String = row.column_text(3).unwrap();
        println!("{}, {}, {}, {}", id, name, age, created);
    }
}

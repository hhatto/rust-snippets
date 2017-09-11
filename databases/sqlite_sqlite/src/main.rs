extern crate sqlite;

use sqlite::Value;

fn main() {
    let conn = sqlite::open(":memory:").expect("sqlite open() error");

    conn.execute("CREATE TABLE person (
                  id        INTEGER PRIMARY KEY,
                  name      TEXT NOT NULL,
                  age       INTEGER NOT NULL,
			      created   TEXT NOT NULL
                 )").expect("create table error.");

    let name = "rustacean";
    let age = 20;
    let mut cursor = conn.prepare("INSERT INTO person (name, age, created)
                                   VALUES (?, ?, datetime())"
                                   ).expect("prepare error").cursor();
    cursor.bind(&[
                Value::String(name.to_string()),
                Value::Integer(age)
    ]).expect("bind insert error");

    while let Some(row) = cursor.next().unwrap() {
        println!("{:?}", row);
    }

    let mut cursor = conn.prepare("SELECT * FROM person").expect("select prepare error").cursor();

    while let Some(row) = cursor.next().unwrap() {
        println!("{:?}", row);
    }
}

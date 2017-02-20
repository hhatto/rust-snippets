extern crate diesel_mysql;
extern crate diesel;

use diesel_mysql::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
    use self::schema::memos::dsl::*;

    let dbconn = establish_connection();
    let results = memos.limit(5).load::<Memo>(&dbconn).expect("select error");

    for memo in results {
        println!("{}, {}, {}", memo.id, memo.title, memo.body);
    }
}

extern crate diesel;
extern crate diesel_mysql;

use self::models::*;
use diesel::prelude::*;
use diesel_mysql::*;

fn main() {
    use self::schema::memos::dsl::*;

    let dbconn = establish_connection();
    let results = memos.limit(5).load::<Memo>(&dbconn).expect("select error");

    for memo in results {
        println!(
            "{}, {}, {}, {}",
            memo.id, memo.title, memo.body, memo.published
        );
    }
}

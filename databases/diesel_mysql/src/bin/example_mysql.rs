extern crate diesel;
extern crate diesel_mysql;

use self::models::*;
use diesel::prelude::*;
use diesel_mysql::*;

fn main() {
    use self::schema::memos::dsl::*;

    let mut dbconn = establish_connection();
    let results = memos.limit(5).load::<Memo>(&mut dbconn).expect("select error");

    for memo in results {
        println!(
            "{}, {}, {}, {}",
            memo.id, memo.title, memo.body, memo.published
        );
    }
}

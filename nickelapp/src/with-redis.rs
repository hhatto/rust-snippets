extern crate r2d2;
extern crate redis;
#[macro_use]
extern crate nickel;
extern crate nickel_redis;
extern crate core;

use std::env;
use r2d2::NopErrorHandler;
use nickel::{Nickel, HttpRouter};
use nickel_redis::{RedisMiddleware, RedisRequestExtensions};
use core::ops::Deref;

fn main() {
    let mut app = Nickel::new();

    let redis_url = env::var("DATABASE_URL").unwrap();
    let dbpool = RedisMiddleware::new(&*redis_url, 5, Box::new(NopErrorHandler)).unwrap();
    app.utilize(dbpool);
    app.get("/my_counter",
            middleware! { |request|
        let pool_con = request.redis_conn();
        let _con = pool_con.deref();

        // Use con as your connection
    });

    app.get("**", middleware! { println!("!!!") });
}

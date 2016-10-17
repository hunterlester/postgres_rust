#[macro_use]
extern crate nickel;
extern crate postgres;
extern crate chrono;

use postgres::{Connection, TlsMode};
use nickel::{Nickel, HttpRouter};
use chrono::{NaiveDate};

struct Herd {
  id: i32,
  breed: String,
  name: String,
  purchase_date: NaiveDate,
}

fn main() {
    let conn = Connection::connect("postgres://postgres:postgres@learn-postgres.c3ccnecqsxt1.us-west-2.rds.amazonaws.com:5432/postgres", TlsMode::None).unwrap();

    let mut server = Nickel::new();

    for row in &conn.query("SELECT * FROM herd", &[]).unwrap() {
      let member = Herd {
        id: row.get(0),
        breed: row.get(1),
        name: row.get(2),
        purchase_date: row.get(3),
      };

      println!("{}, {}, {}, {}", member.id, member.breed, member.name, member.purchase_date)
    }

    server.utilize(middleware! { |req|
      println!("{} => {:?}", req.origin.method, req.origin.uri);
    });

    server.get("**", middleware!("Hello with Nickel!"));

    server.listen("127.0.0.1:6767");
}

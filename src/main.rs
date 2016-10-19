#[macro_use]
extern crate nickel;
extern crate postgres;
extern crate chrono;
extern crate rustc_serialize;

use postgres::{Connection, TlsMode};
use postgres::rows::Row;
use nickel::{Nickel, HttpRouter, Response, Responder, MiddlewareResult, StaticFilesHandler};
use chrono::{NaiveDate};
use std::collections::{BTreeMap, HashMap};
use rustc_serialize::json::{self, Json, ToJson};

#[derive(Debug)]
struct Herd {
  id: i32,
  breed: String,
  name: String,
  purchase_date: NaiveDate,
}

impl<D> Responder<D> for Herd {
    fn respond<'a>(self, response: Response<'a, D>) -> MiddlewareResult<'a, D> {
        response.send(self.to_json())
    }
}

impl ToJson for Herd {
  fn to_json(&self) -> json::Json {
    let mut member = BTreeMap::new();
    member.insert("id".to_string(), self.id.to_json());
    member.insert("breed".to_string(), self.breed.to_json());
    member.insert("name".to_string(), self.name.to_json());
    member.insert("purchase_date".to_string(), self.purchase_date.to_string().to_json());

    Json::Object(member)
  }
}

fn retrieve_row(row: Row) -> Herd {
  Herd {
    id: row.get(0),
    breed: row.get(1),
    name: row.get(2),
    purchase_date: row.get(3),
  }
}

fn main() {
    let conn = Connection::connect("postgres://postgres:postgres@learn-postgres.c3ccnecqsxt1.us-west-2.rds.amazonaws.com:5432/postgres", TlsMode::None).unwrap();

    let mut server = Nickel::new();

    let data: Vec<Herd> = conn.query("SELECT * FROM herd", &[]).unwrap().into_iter().map(retrieve_row).collect();

    //println!("{:?}", &data);

    server.utilize(StaticFilesHandler::new("src/assets/"));

    server.utilize(middleware! { |req|
      println!("{} => {:?}", req.origin.method, req.origin.uri);
    });

    server.utilize(router! {
      get "/" => |req, res| {
        let mut view_data = HashMap::<&str, &str>::new();
        view_data.insert("name", "user");
        return res.render("src/assets/index.tpl", &view_data)
      }

      get "/herd" => |req, res| {
        data.to_json()
      }
    });

    server.listen("127.0.0.1:6767").unwrap();
}

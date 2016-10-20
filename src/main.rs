#[macro_use]
extern crate nickel;
extern crate postgres;
extern crate chrono;
extern crate rustc_serialize;

use postgres::{Connection, TlsMode};
use postgres::rows::Row;
use std::io;
use nickel::{Nickel, HttpRouter, Request, Response, Responder, MiddlewareResult, StaticFilesHandler, JsonBody};
use chrono::{NaiveDate};
use std::collections::{BTreeMap, HashMap};
use rustc_serialize::json::{self, Json, ToJson};
use rustc_serialize::{Decodable, Decoder};
use nickel::status::StatusCode;

#[derive(Debug, RustcDecodable)]
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

fn parse_todo(request: &mut Request) -> Result<Herd, (StatusCode, io::Error)> {

    request.json_as().map_err(|e| (StatusCode::BadRequest, e))

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

      post "/herd" => |req, res| {
      // let req_data: BTreeMap<Request> = req.json_as().unwrap();
      // let post_data = conn.query("INSERT INTO herd (breed, name, purchase_date) VALUES ($1, $2, $3)", &[&req_data.body.breed, &req_data.body.name, &req_data.body.purchase_date]).unwrap().into_iter().collect();

      // println!("{:?}", &post_data);
      // return "hunterlj"

      // let mut post_data: BTreeMap<String, Json> = req.json_as().unwrap();
      // println!("{:?}", post_data)

      let data = try_with!(res, parse_todo(req));
      println!("{:?}", &data);
      return res.send(&data)
      }
    });

    server.listen("127.0.0.1:6767").unwrap();
}

use serde::{Deserialize, Serialize};
use anyhow::Result;
use http::*;
use kv::*;
use slight_http_handler_macro::register_handler;
use std::str;

wit_bindgen_rust::import!("wit/http.wit");
wit_bindgen_rust::import!("wit/kv.wit");
wit_error_rs::impl_error!(http::Error);
wit_error_rs::impl_error!(kv::Error);

#[derive(Serialize, Deserialize)]
struct KVDTO {
  key: String,
  value: String,
}

#[derive(Serialize, Deserialize)]
struct KDTO {
  key: String,
}

fn main() -> Result<()> {
  let router = Router::new()?;
  let router_with_route = router
    .get("/read", "get")?
    .put("/create", "put")?
    .post("/update", "post")?
    .delete("/delete", "delete")?;

  println!("Starting server...");
  let _ = Server::serve("0.0.0.0:3000", &router_with_route)?;
  println!("Listening on address: 0.0.0.0 port: 3000");
  Ok(())
}

// curl http://localhost:3000/read
#[register_handler]
fn get(req: Request) -> Result<Response, Error> {
  assert_eq!(req.method, Method::Get);
  let headers: String = req
    .headers
    .into_iter()
    .map(|x| format!("{}: {}\r\n", x.0, x.1))
    .collect();
  let newline = "\r\n";
  let mut body = (headers + &newline).as_bytes().to_vec();
  body.append(&mut req.body.unwrap_or_else(|| vec![]));
  let kv = crate::Kv::open("rust-slight").unwrap();
  let keys = kv.keys().unwrap();
  for key in keys.iter() {
    let s_key = &key.replace("rust-slight:", "");
    body.append(&mut s_key.as_bytes().to_vec());
    body.append(&mut ": ".as_bytes().to_vec());
    body.append(&mut kv.get(&s_key).unwrap());
    body.append(&mut newline.as_bytes().to_vec());
  }
  Ok(Response {
    headers: None,
    body: Some(body),
    status: 200,
  })
}

// curl -X PUT -d '{"key": "version", "value": "1.0.0"}' http://localhost:3000/create
#[register_handler]
fn put(req: Request) -> Result<Response, Error> {
  assert_eq!(req.method, Method::Put);
  let data: crate::KVDTO = serde_json::from_slice(&req.body.unwrap()).unwrap();
  let kv = crate::Kv::open("rust-slight").unwrap();
  kv.set(&data.key, data.value.as_bytes()).unwrap();
  Ok(Response {
    headers: None,
    body: None,
    status: 200,
  })
}

// curl -X POST -d '{"key": "version", "value": "1.0.1"}' http://localhost:3000/update
#[register_handler]
fn post(req: Request) -> Result<Response, Error> {
  assert_eq!(req.method, Method::Post);
  let data: crate::KVDTO = serde_json::from_slice(&req.body.unwrap()).unwrap();
  let kv = crate::Kv::open("rust-slight").unwrap();
  kv.set(&data.key, data.value.as_bytes()).unwrap();
  Ok(Response {
    headers: None,
    body: None,
    status: 200,
  })
}

// curl -X DELETE -d '{"key": "version"}' http://localhost:3000/delete
#[register_handler]
fn delete(req: Request) -> Result<Response, Error> {
  assert_eq!(req.method, Method::Delete);
  let data: crate::KDTO = serde_json::from_slice(&req.body.unwrap()).unwrap();
  let kv = crate::Kv::open("rust-slight").unwrap();
  kv.delete(&data.key).unwrap();
  Ok(Response {
    headers: None,
    body: None,
    status: 200,
  })
}

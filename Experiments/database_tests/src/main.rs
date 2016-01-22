extern crate iron;
extern crate mysql;
use std::default::Default;
use std::str::FromStr;
use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_row;
use mysql::value::Value;
use mysql::value::FromValue;

use std::env;
use iron::prelude::*;
use iron::status;

#[derive(Debug, PartialEq, Eq)]
struct Account {
    id: i64,
    name: Option<String>,
    subdomain: Option<String>,
    owner_id: Option<i64>
}

fn main() {
  let opts = MyOpts {
    user: Some("user".to_string()),
    pass: Some("password".to_string()),
    db_name: Some("db_name".to_string()),
    tcp_addr: Some("address".to_string()),
    tcp_port: 1234,
    ..Default::default()
  };
  let pool = MyPool::new_manual(1, 1,opts).unwrap();
  let result = pool.prep_exec("SELECT id, name, subdomain, owner_id from accounts", ()).unwrap();
  let accounts: Vec<Account> = result.map(|row| {
    let (id, name, subdomain, owner_id) = from_row(row.unwrap());
      Account {
        id: id,
        name: name,
        subdomain: subdomain,
        owner_id: owner_id
      }
  }).collect();

  println!("{:?}", accounts);
}

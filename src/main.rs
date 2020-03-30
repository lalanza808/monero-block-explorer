#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate reqwest;
mod data_types;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use reqwest::blocking::RequestBuilder;
use std::env;
use data_types::*;

fn issue_rpc(method: &str, params: Option<RPCParams>) -> RequestBuilder {
    let http_client = reqwest::blocking::Client::new();
    let url = format!(
        "{}/json_rpc",
        env::var("DAEMON_URI").unwrap()
    );
    let post_data = RPCPayload {
        method: method.to_string(),
        params: params,
        ..Default::default()
    };
    http_client.post(&url).json(&post_data)
}

fn issue_raw_rpc(method: &str, params: JsonValue) -> RequestBuilder {
    let http_client = reqwest::blocking::Client::new();
    let url = format!(
        "{}/{}",
        env::var("DAEMON_URI").unwrap(),
        &method
    );
    http_client.post(&url).json(&params)
}

// /block

#[get("/hash/<block_hash>")]
fn get_block_header_by_block_hash(block_hash: String) -> Json<BlockHeader> {
    let params = RPCParams {
        hash: Some(block_hash),
        ..Default::default()
    };
    let res: BlockByHeaderHash = issue_rpc(&"get_block_header_by_hash", Some(params))
        .send().unwrap().json().unwrap();
    Json(res.result.block_header)
}

#[get("/tx/<tx_hash>")]
fn get_block_header_by_transaction_hash(tx_hash: String) -> Json<GetTransactions> {
    let params: JsonValue = json!({"txs_hashes": [&tx_hash]});
    let res: GetTransactions = issue_raw_rpc(&"get_transactions", params)
        .send().unwrap().json().unwrap();
    Json(res)
}

// /

#[get("/info")]
fn get_daemon_info() -> Json<GetInfoResult> {
    let res: GetInfo = issue_rpc(&"get_info", None)
        .send().unwrap().json().unwrap();
    Json(res.result)
}

#[post("/", format = "application/x-www-form-urlencoded")]
fn something() -> Template {
    let res: GetInfo = issue_rpc(&"get_info", None)
        .send().unwrap().json().unwrap();
    Template::render("search", &res.result)
}

#[get("/")]
fn index() -> Template {
    let res: GetInfo = issue_rpc(&"get_info", None)
        .send().unwrap().json().unwrap();
    Template::render("index", &res.result)
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() {
    let env_url = env::var("DAEMON_URI");
    match env_url {
        Ok(_) => {
            rocket::ignite()
                .mount("/", routes![
                    index, something,
                    get_daemon_info
                ])
                .mount("/block", routes![
                    get_block_header_by_block_hash,
                    get_block_header_by_transaction_hash
                ])
                .mount("/static", StaticFiles::from("./static"))
                .register(catchers![not_found])
                .attach(Template::fairing())
                .launch();
        },
        Err(_) => panic!("Environment variable `DAEMON_URI` not provided.")
    }
}

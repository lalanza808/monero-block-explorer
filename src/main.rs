#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate reqwest;
mod data_types;
use rocket_contrib::json::{Json, JsonValue};
use std::env;
use data_types::*;



fn issue_rpc(method: &str) -> reqwest::blocking::RequestBuilder {
    let uri = env::var("DAEMON_URI").unwrap();
    let http_client = reqwest::blocking::Client::new();
    let post_data = RPCPayload {
        method: method.to_string(),
        ..Default::default()
    };
    return http_client.post(&uri).json(&post_data);
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[get("/transaction/<hash>")]
fn get_transaction(hash: String) -> Json<BlockHeader> {
    let res: BlockByHeaderHash = issue_rpc(&"get_block_header_by_hash")
        .send()
        .unwrap().json().unwrap();
    Json(res.result.block_header)
}

#[get("/info")]
fn get_daemon_info() -> Json<GetInfoResult> {
    let res: GetInfo = issue_rpc(&"get_info")
        .send()
        .unwrap().json().unwrap();
    Json(res.result)
}

fn main() {
    let env_url = env::var("DAEMON_URI");
    match env_url {
        Ok(_) => {
            rocket::ignite()
                .mount("/", routes![
                    get_daemon_info, get_transaction
                ])
                .register(catchers![not_found])
                .launch();
        },
        Err(_) => panic!("Environment variable `DAEMON_URI` not provided.")
    }
}

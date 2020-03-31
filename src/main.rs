#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate reqwest;
mod data_types;
use rocket::http::RawStr;
use rocket::response::Redirect;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use reqwest::blocking::{RequestBuilder, Client};
use std::env;
use data_types::*;

fn issue_rpc(method: &str, params: Option<RPCParams>) -> RequestBuilder {
    let http_client = Client::new();
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
    let http_client = Client::new();
    let url = format!(
        "{}/{}",
        env::var("DAEMON_URI").unwrap(),
        &method
    );
    http_client.post(&url).json(&params)
}

#[get("/block/hash/<block_hash>")]
fn get_block_header_by_block_hash(block_hash: String) -> Json<BlockHeader> {
    let params = RPCParams {
        hash: Some(block_hash),
        ..Default::default()
    };
    let res: BlockByHeaderHash = issue_rpc(&"get_block_header_by_hash", Some(params))
        .send().unwrap().json().unwrap();
    Json(res.result.block_header)
}

#[get("/block/height/<block_height>")]
fn get_block_by_height(block_height: String) -> String {
    let params = RPCParams {
        height: Some(block_height),
        ..Default::default()
    };
    let res: GetBlock = issue_rpc(&"get_block", Some(params))
        .send().unwrap().json().unwrap();
    // Json(res.result.block_header)
    serde_json::to_string(&res).unwrap()
}

#[get("/transaction/<tx_hash>")]
fn get_block_header_by_transaction_hash(tx_hash: String) -> Json<GetTransactions> {
    let params: JsonValue = json!({"txs_hashes": [&tx_hash]});
    let res: GetTransactions = issue_raw_rpc(&"get_transactions", params)
        .send().unwrap().json().unwrap();
    Json(res)
}

#[get("/search?<value>")]
fn search(value: &RawStr) -> Redirect {
    let sl: usize = value.len();
    let first_byte = value.get(0..1).unwrap();
    println!("Search value: {}", value);
    println!("First byte: {}", first_byte);

    if sl < 10 {
        match value.parse::<u32>() {
            Ok(v) => {
                println!("Found: {}", v);
                // "this looks like a block height"
                return Redirect::found(uri!(get_block_by_height: value.as_str()));
            },
            Err(e) => {
                println!("Error: {}", e);
                // "this is an invalid search query"
                return Redirect::found(uri!(index));
            }
        }
    } else if sl < 95 {
        // "this looks like a tx or block hash"
        return Redirect::found(uri!(index));
    } else if sl == 95 {
        match first_byte {
            "9" => {
                println!("This looks like a testnet address");
                return Redirect::found(uri!(index));
            },
            "A" => {
                println!("This looks like a testnet subaddress");
                return Redirect::found(uri!(index));
            },
            "5" => {
                println!("This looks like a stagenet address");
                return Redirect::found(uri!(index));
            },
            "7" => {
                println!("This looks like a stagenet subaddress");
                return Redirect::found(uri!(index));
            },
            "4" => {
                println!("This looks like a mainnet address");
                return Redirect::found(uri!(index));
            },
            "8" => {
                println!("This looks like a mainnet subaddress");
                return Redirect::found(uri!(index));
            },
            _ => {
                println!("Not sure what this is");
                return Redirect::found(uri!(index));
            }
        }
    } else if sl == 105 {
        // "this looks like an integrated address"
        return Redirect::found(uri!(index));
    } else {
        // "no idea what this is"
        return Redirect::found(uri!(index));
    };

    // println!("No if stmt matched");
    // return Redirect::found(uri!(index));
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
                    index,
                    search,
                    get_block_by_height,
                    get_block_header_by_block_hash,
                    get_block_header_by_transaction_hash,
                ])
                .mount("/static", StaticFiles::from("./static"))
                .register(catchers![not_found])
                .attach(Template::fairing())
                .launch();
        },
        Err(_) => panic!("Environment variable `DAEMON_URI` not provided.")
    }
}

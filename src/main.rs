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
use reqwest::Error;
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
fn get_block_by_hash(block_hash: String) -> Template {
    let params = RPCParams {
        hash: Some(block_hash),
        ..Default::default()
    };
    let res: GetBlock = issue_rpc(&"get_block", Some(params))
        .send().unwrap().json().unwrap();
    Template::render("block", &res.result)
}

#[get("/block/height/<block_height>")]
fn get_block_by_height(block_height: String) -> Template {
    let params = RPCParams {
        height: Some(block_height),
        ..Default::default()
    };
    let res: GetBlock = issue_rpc(&"get_block", Some(params))
        .send().unwrap().json().unwrap();
    Template::render("block", &res.result)
}

#[get("/transaction/<tx_hash>")]
fn get_transaction_by_hash(tx_hash: String) -> Json<GetTransactions> {
    let params: JsonValue = json!({"txs_hashes": [&tx_hash]});
    let res: GetTransactions = issue_raw_rpc(&"get_transactions", params)
        .send().unwrap().json().unwrap();
    Json(res)
}

#[get("/search?<value>")]
fn search(value: &RawStr) -> Redirect {
    // This search implementation is not ideal but it works.
    // We basically check the length of the search value and
    // attempt to redirect to the appropriate route.
    let sl: usize = value.len();
    if sl < 8 {
        // Less than 8 characters is probably a block height. If it can
        // be parsed as valid u32 then redirect to `get_block_by_height`,
        // otherwise redirect to the error response.
        match value.parse::<u32>() {
            Ok(_) => return Redirect::found(uri!(get_block_by_height: value.as_str())),
            Err(_) => return Redirect::found(uri!(error))
        }
    } else if sl == 64 {
        // Equal to 64 characters is probably a hash; block or tx.
        // For this we attempt to query for a block with
        // given hash. If we don't receive a valid/expected
        // response then we attempt to query for a transaction hash.
        // If neither works then redirect to error response.
        let block_hash_params = RPCParams {
            hash: Some(value.to_string()),
            ..Default::default()
        };
        let check_valid_block_hash: Result<GetBlock, Error> = issue_rpc(
            &"get_block", Some(block_hash_params)
        ).send().unwrap().json();

        match check_valid_block_hash {
            Ok(_) => return Redirect::found(uri!(get_block_by_hash: value.as_str())),
            Err(_) => {
                let tx_hash_params: JsonValue = json!({"txs_hashes": [&value.as_str()]});
                let check_valid_tx_hash: Result<GetTransactions, Error> = issue_raw_rpc(
                    &"get_transactions", tx_hash_params
                ).send().unwrap().json();

                match check_valid_tx_hash {
                    Ok(_) => return Redirect::found(uri!(get_transaction_by_hash: value.as_str())),
                    Err(_) => return Redirect::found(uri!(error))
                }
            }
        }
    } else {
        // Anything else hasn't been implemented yet
        // so redirect to error response.
        return Redirect::found(uri!(error));
    };
}

#[get("/")]
fn index() -> Template {
    let res: GetInfo = issue_rpc(&"get_info", None)
        .send().unwrap().json().unwrap();
    Template::render("index", &res.result)
}

#[get("/error", )]
fn error() -> JsonValue {
    json!({
        "status": "error",
        "reason": "There was an error while searching the provided values."
    })
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
                    get_block_by_hash,
                    get_transaction_by_hash,
                    error
                ])
                .mount("/static", StaticFiles::from("./static"))
                .register(catchers![not_found])
                .attach(Template::fairing())
                .launch();
        },
        Err(_) => panic!("Environment variable `DAEMON_URI` not provided.")
    }
}

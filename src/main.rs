#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate reqwest;
extern crate qrcode_generator;
mod data_types;

use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::json::{Json,JsonValue};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use reqwest::blocking::{RequestBuilder, Client};
use reqwest::Error;
use qrcode_generator::QrCodeEcc;
use std::env;
use data_types::*;

fn issue_rpc(method: &str, params: Option<RPCParams>) -> RequestBuilder {
    let http_client = Client::new();
    let url = format!(
        "{}/json_rpc",
        env::var("DAEMON_URI").unwrap()
    );
    let post_data = RPCPayload {
        method: Some(method.to_string()),
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

fn build_rpc(method: &str, data: Option<JsonValue>, raw: bool) -> RequestBuilder {
    let http_client = Client::new();
    let daemon_uri = env::var("DAEMON_URI").unwrap();
    match raw {
        true => {
            let uri = format!("{}/{}", &daemon_uri, &method);
            if let None = data {
                http_client.post(&uri)
            } else {
                http_client.post(&uri).json(&data)
            }
        },
        false => {
            let uri = format!("{}/json_rpc", &daemon_uri);
            let data = RPCPayload {
                method: Some(method.to_string()),
                ..Default::default()
            };
            http_client.post(&uri).json(&data)
        }
    }
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
fn get_transaction_by_hash(tx_hash: String) -> Template {
    let params: JsonValue = json!({
        "txs_hashes": [&tx_hash],
        "decode_as_json": true
    });
    let mut res: GetTransactions = issue_raw_rpc(&"get_transactions", params)
        .send().unwrap().json().unwrap();
    for f in &mut res.txs {
        f.process();
    };
    let context = json!({
        "tx_info": res.txs,
        "tx_hash": tx_hash
    });
    Template::render("transaction", context)
}

#[get("/address/<wallet_address>?<tx_amount>&<tx_description>&<recipient_name>&<tx_payment_id>")]
fn show_wallet_address(
        wallet_address: String,
        tx_amount: Option<String>,
        tx_description: Option<String>,
        recipient_name: Option<String>,
        tx_payment_id: Option<String>
    ) -> Template {
    let address_uri = format!(
        "monero:{}&tx_amount={}&tx_description={}&recipient_name={}&tx_payment_id={}",
        wallet_address,
        tx_amount.unwrap_or("".to_string()),
        tx_description.unwrap_or("".to_string()),
        recipient_name.unwrap_or("".to_string()),
        tx_payment_id.unwrap_or("".to_string())
    );
    let qr_code: String = qrcode_generator::to_svg_to_string(address_uri, QrCodeEcc::Low, 256, None)
        .unwrap();
    let qr_code: String = base64::encode(qr_code);
    let context: JsonValue = json!({
        "qr_code": qr_code,
        "wallet_address": wallet_address
    });
    Template::render("address", context)
}

#[get("/search?<value>")]
fn search(value: &RawStr) -> Redirect {
    // This search implementation is not ideal but it works.
    // We basically check the length of the search value and
    // attempt to redirect to the appropriate route.
    let sl: usize = value.len();
    if sl == 0 {
        return Redirect::found(uri!(index));
    } else if sl < 8 {
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
    } else if sl == 95 {
        // Equal to 95 characters is probably a wallet address.
        // For this let's just redirect to the `show_wallet_address` route.
        return Redirect::found(uri!(show_wallet_address: value.as_str(), "", "", "", ""))
    } else if sl == 105 {
        // Equal to 105 characters is probably an integrated address.
        // For this let's just redirect to the `show_wallet_address` route.
        return Redirect::found(uri!(show_wallet_address: value.as_str(), "", "", "", ""))
    } else {
        // Anything else hasn't been implemented yet
        // so redirect to error response.
        return Redirect::found(uri!(error));
    };
}

#[get("/tx_pool")]
fn show_tx_pool() -> Json<GetTransactionPool> {
    let mut tx_pool: GetTransactionPool = build_rpc(
        &"get_transaction_pool", None, true
    ).send().unwrap().json().unwrap();

    for f in &mut tx_pool.transactions {
        f.process();
    };

    Json(tx_pool)
}

#[get("/")]
fn index() -> Template {
    let daemon_info: GetInfo = issue_rpc(&"get_info", None)
        .send().unwrap().json().unwrap();

    let mut tx_pool: GetTransactionPool = build_rpc(
        &"get_transaction_pool", None, true
    ).send().unwrap().json().unwrap();

    let mut tx_json_raw: Vec<TransactionJSON> = vec![];

    for f in &mut tx_pool.transactions {
        f.process();
        let j: TransactionJSON = serde_json::from_str(&f.tx_json).unwrap();
        tx_json_raw.push(j)
    };

    let context: JsonValue = json!({
        "daemon_info": daemon_info.result,
        "tx_pool": tx_pool.transactions,
        "tx_json": tx_json_raw
    });

    Template::render("index", context)
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
                    show_tx_pool,
                    get_block_by_height,
                    get_block_by_hash,
                    get_transaction_by_hash,
                    show_wallet_address,
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

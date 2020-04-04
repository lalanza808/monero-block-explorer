#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct RPCPayload {
    pub jsonrpc: String,
    pub id: String,
    pub method: Option<String>,
    pub params: Option<RPCParams>
}

impl Default for RPCPayload {
    fn default() -> RPCPayload {
        RPCPayload {
            jsonrpc: "2.0".to_string(),
            id: "0".to_string(),
            method: None,
            params: None,
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct RPCParams {
    pub hash: Option<String>,
    pub txs_hashes: Option<Vec<String>>,
    pub height: Option<String>
}

impl Default for RPCParams {
    fn default() -> RPCParams {
        RPCParams {
            hash: None,
            txs_hashes: None,
            height: None
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct GetInfo {
    pub jsonrpc: String,
    pub id: String,
    pub result: GetInfoResult
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct GetInfoResult {
    pub alt_blocks_count: u32,
    pub block_size_limit: u32,
    pub block_size_median: u32,
    pub block_weight_limit: u32,
    pub block_weight_median: u64,
    pub bootstrap_daemon_address: String,
    pub credits: u32,
    pub cumulative_difficulty: u64,
    pub cumulative_difficulty_top64: u32,
    pub database_size: u64,
    pub difficulty: u64,
    pub difficulty_top64: u32,
    pub free_space: u64,
    pub grey_peerlist_size: u32,
    pub height: u32,
    pub height_without_bootstrap: u32,
    pub incoming_connections_count: u32,
    pub mainnet: bool,
    pub nettype: String,
    pub offline: bool,
    pub outgoing_connections_count: u32,
    pub rpc_connections_count: u32,
    pub stagenet: bool,
    pub start_time: u32,
    pub status: String,
    pub target: u32,
    pub target_height: u32,
    pub testnet: bool,
    pub top_block_hash: String,
    pub top_hash: String,
    pub tx_count: u32,
    pub tx_pool_size: u32,
    pub untrusted: bool,
    pub update_available: bool,
    pub version: String,
    pub was_bootstrap_ever_used: bool,
    pub white_peerlist_size: u32,
    pub wide_cumulative_difficulty: String,
    pub wide_difficulty: String
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct BlockByHeaderHash {
    pub id: String,
    pub jsonrpc: String,
    pub result: BlockByHeaderHashResult
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct BlockByHeaderHashResult {
    pub status: String,
    pub untrusted: bool,
    pub block_header: BlockHeader
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct BlockHeader {
    pub block_size: u32,
    pub depth: u32,
    pub difficulty: u64,
    pub hash: String,
    pub height: u32,
    pub major_version: u8,
    pub minor_version: u8,
    pub miner_tx_hash: Option<String>,
    pub nonce: u32,
    pub num_txes: u32,
    pub orphan_status: bool,
    pub prev_hash: String,
    pub reward: u64,
    pub timestamp: i64
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct GetTransactions {
    pub txs: Vec<GetTransactionsTxs>
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct GetTransactionsTxs {
    pub block_height: u32,
    pub block_timestamp: i64,
    pub double_spend_seen: bool,
    pub in_pool: bool,
    pub output_indices: Vec<u32>,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct GetBlock {
    pub id: String,
    pub jsonrpc: String,
    pub result: GetBlockResult
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct GetBlockResult {
    pub block_header: BlockHeader,
    pub credits: u8,
    pub miner_tx_hash: String,
    pub tx_hashes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct GetTransactionPool {
    pub credits: u32,
    pub spent_key_images: Vec<SpentKeyImages>,
    pub status: String,
    pub top_hash: String,
    pub transactions: Vec<Transactions>
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct SpentKeyImages {
    pub id_hash: String,
    pub txs_hashes: Vec<String>
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct Transactions {
    pub blob_size: u32,
    pub do_not_relay: bool,
    pub double_spend_seen: bool,
    pub fee: u64,
    pub id_hash: String,
    pub kept_by_block: bool,
    pub last_failed_height: u32,
    pub last_failed_id_hash: String,
    pub last_relayed_time: i64,
    pub max_used_block_height: u32,
    pub max_used_block_id_hash: String,
    pub receive_time: i64,
    pub relayed: bool,
    pub tx_blob: String,
    pub tx_json: String,
    pub weight: u32
}

// #[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
// pub struct TransactionJSON {
//     pub version: u32,
//     pub unlock_time: u64,
//     pub vin: TransactionInputs,
//     pub vout: TransactionOutputs,
//     pub extra: String,
//     pub signatures: Vec<String>
// }
//
// #[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
// pub struct TransactionInputs {
//     pub pubkey: PreviousTransactionKey
// }
//
// #[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
// pub struct PreviousTransactionKey {
//     pub amount: u32,
//     pub key_offsets: Vec<u32>,
//     pub k_image: String
// }
//
// #[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
// pub struct TransactionOutputs {
//     pub amount: u32,
//     pub target: OutputStealthAddress
// }
//
// #[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
// pub struct OutputStealthAddress {
//     pub key: String
// }

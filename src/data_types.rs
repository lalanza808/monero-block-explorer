#[derive(Serialize, Deserialize)]
pub struct RPCPayload {
    pub jsonrpc: String,
    pub id: String,
    pub method: String
}

impl Default for RPCPayload {
    fn default() -> RPCPayload {
        RPCPayload {
            jsonrpc: "2.0".to_string(),
            id: "0".to_string(),
            method: "get_info".to_string()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetInfo {
    pub jsonrpc: String,
    pub id: String,
    pub result: GetInfoResult
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct BlockByHeaderHash {
    pub id: String,
    pub jsonrpc: String,
    pub result: BlockByHeaderHashResult
}

#[derive(Serialize, Deserialize)]
pub struct BlockByHeaderHashResult {
    pub status: String,
    pub untrusted: bool,
    pub block_header: BlockHeader
}

#[derive(Serialize, Deserialize)]
pub struct BlockHeader {
    pub block_size: u32,
    pub depth: u32,
    pub difficulty: u64,
    pub hash: String,
    pub height: u32,
    pub major_version: u8,
    pub minor_version: u8,
    pub nonce: u32,
    pub num_txes: u32,
    pub orphan_status: bool,
    pub prev_hash: String,
    pub reward: u64,
    pub timestamp: u64
}

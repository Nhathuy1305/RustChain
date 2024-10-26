use std::collections::hash_map::Values;
use std::collections::HashMap;

const BLOCK_VALID_CHAIN: u64 = 4;
const BLOCK_HAVE_DATA: u64 = 8;

// Holds the index of longest valid chain
pub struct ChainIndex {
    max_height: u64,
    block_index: HashMap<u64, BlockIndexRecord>,
    max_height_blk_index: HashMap<u64, u64>,    // Maps blk_index to max_height found in the file
}

// Holds the metadata where the block data is stored,
// See https://bitcoin.stackexchange.com/questions/28168/what-are-the-keys-used-in-the-blockchain-leveldb-ie-what-are-the-keyvalue-pair
pub struct BlockIndexRecord {
    pub block_hash: sha256d::Hash,
    pub blk_index: u64,
    pub data_offset: u64,   // offset within the blk file
    version: u64,
    height: u64,
    status: u64,
    tx_count: u64,
}

// impl ChainIndex {
//     pub fn new(options: &ParserOptions) -> OpRe
// }

// impl BlockIndexRecord {
//     fn from(key: &[u8], values: &[u8] -> )
// }
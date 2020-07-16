use sha2::Digest;
use serde::Deserialize;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json::json;

use crate::transaction::*;


/// ```rust
/// use bc::block::*;
/// let x = "hello".to_string();
/// assert_eq!(
///   string2hash_string(&x),
///   "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
///  );
/// ```
pub fn string2hash_string(string: &std::string::String)-> std::string::String {
    format!(
        "{:x}", sha2::Sha512::digest(string.as_bytes())
        )
}

#[derive(Debug, Clone, Deserialize)]
pub struct Block {
    pub index: u64,
    pub time: i64,
    pub proof: i64,
    pub previous_hash: std::string::String,
    pub transactions: Vec<Transaction>,
}

impl Block {

    pub fn to_json(&self) -> std::string::String{
        let a = json!({
            "index": self.index,
            "previous_hash": self.previous_hash,
            "proof": self.proof,
            "time": self.time,
            "transactions": self.transactions,
        }).to_string();
        a

    }

}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 5 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Block", 5)?;
        state.serialize_field("index", &self.index)?;
        state.serialize_field("time", &self.time)?;
        state.serialize_field("proof", &self.proof)?;
        state.serialize_field("previous_hash", &self.previous_hash)?;
        state.serialize_field("transactions", &self.transactions)?;
        state.end()
    }
}


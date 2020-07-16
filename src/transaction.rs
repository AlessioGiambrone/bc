//use sha2::Digest;
//use chrono::{prelude::*};
use serde::Deserialize;
use serde::ser::{Serialize, Serializer, SerializeStruct};
//use serde_json::json;

#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Transaction", 4)?;
        state.serialize_field("sender", &self.sender)?;
        state.serialize_field("amount", &self.amount)?;
        state.serialize_field("receiver", &self.receiver)?;
        state.end()
    }
}

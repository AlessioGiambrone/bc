use chrono::{prelude::*};
use std::collections::HashSet;
use std::convert::TryFrom;
use url::Url;

use crate::block::*;
use crate::transaction::*;

pub struct Blockchain{
    pub chain: Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub nodes: HashSet<String>,
    me: String,
    mining_fee: u64,
    node_address: String,
}

impl Blockchain {

    pub fn new(node_address:String, me: String, mining_fee: u64) -> Blockchain {
        let transactions: Vec<Transaction> = Vec::new();
        println!("address: {}", node_address);
        let mut bc = Blockchain {
            chain: Vec::new(),
            transactions: transactions.clone(),
            nodes: HashSet::new(),
            me,
            mining_fee,
            node_address,
        };
        bc.create_block(1, "0".to_string(), transactions);
        bc
    }

    pub fn create_block(
        &mut self,
        proof: i64,
        previous_hash:  std::string::String,
        transactions: Vec<Transaction>,
    ) -> &Block {
        self.chain.push(Block{
            index: u64::try_from( self.chain.len()).unwrap(), 
            time: Utc::now().timestamp(),
            proof,
            previous_hash,
            transactions: transactions,
        });
        self.get_last_block()
    }

    fn get_last_proof(&self) -> (i64, Block) {
        // a first block always exists.
        let previous_block = self.chain.last().unwrap().clone();
        (previous_block.proof, previous_block)
    }

    pub fn mine_block(&mut self) -> &Block{
        let last_proof = self.get_last_proof();
        self.add_transaction(
            self.node_address.clone(), self.me.clone(), self.mining_fee);
        let transactions = self.transactions.clone();
        let proof = Blockchain::proof_of_work(last_proof.0);
        self.transactions = Vec::new();
        self.create_block(
            proof,
            Blockchain::hash(&last_proof.1),
            transactions)
    }

    pub fn get_last_block(&mut self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn proof_function(new_proof: i64, previous_proof: i64) -> bool {
        let new_proof_i = new_proof.pow(2)-previous_proof.pow(2);
        let new_proof_str = &format!("{}", new_proof_i);
        let new_proof_hash = string2hash_string(new_proof_str);
        if new_proof_hash.split_at(4).0 == "0000" {
            return true
        }
        false
    }

    pub fn proof_of_work(previous_proof: i64) -> i64{
        let mut new_proof: i64 = 1;
        let mut check_proof: bool = false;
        while check_proof != true {
            if Blockchain::proof_function(new_proof, previous_proof){
                check_proof = true;
            }
            else {
                new_proof += 1;
            }
        }
        new_proof
    }

    pub fn hash(block: &Block) -> std::string::String {
        string2hash_string(&block.to_json())
    }

    // TODO is it really needed to pass chain argument??? Doesn't seem so.
    pub fn is_chain_valid(chain: &Vec<Block>) -> bool{
        let mut block_index = 1;
        let mut previous_block = &chain[block_index - 1];
        while block_index < chain.len() {
            let block = &chain[block_index];
            if !block.previous_hash.eq(&Blockchain::hash(&previous_block)) {
                return false
            }
            let previous_proof = previous_block.proof;
            let proof = block.proof;
            if !Blockchain::proof_function(proof, previous_proof){
                return false
            }
            previous_block = block;
            block_index += 1
        }
        true
    }

    pub fn add_transaction(
        &mut self, sender: String, receiver: String, amount: u64
    ) -> u64 {
        self.transactions.push(Transaction{
            sender,
            receiver,
            amount
        });
        self.get_last_block().index + 1
    }

    pub fn add_node(&mut self, address: String) -> Result<(), url::ParseError> {
        let url = Url::parse(&address)?;
        self.nodes.insert(url.as_str().to_string());
        Ok(())
    }

    fn test_remote_chain(remote_url: &String, max_size: usize)
        -> Result<(bool, Vec<Block>), reqwest::Error>
    {
        let mut result = Ok((false, Vec::new()));
        let mut url = remote_url.clone();
        url.push_str("blockchain_length");
        let mut response: reqwest::Response = reqwest::get(&url)?;
        if response.status().is_success(){
            let length = response.text()?.parse::<u64>().unwrap();
            if length > max_size as u64{
                url = remote_url.clone();
                url.push_str("blockchain");
                let mut response_chain = reqwest::get(&url)?;
                if response_chain.status().is_success(){
                    let new_chain: Vec<Block> = response_chain.json()?;
                    result = Ok((
                        Blockchain::is_chain_valid(&new_chain),
                        new_chain
                    ))
                }
            }
        } 
        result
    }

    pub fn replace_chain(&mut self) -> bool {
        let mut max_size = self.chain.len();
        let original_size = max_size;
        for node in &self.nodes {
            let check_result = Blockchain::test_remote_chain(node, max_size);
            match check_result {
                Ok(r) => if r.0 {
                    max_size = r.1.len();
                    self.chain = r.1;
                },
                Err(e) => println!("Error while calling {}: {}", node, e)
            }
        };
        max_size > original_size
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_chain_valid() {
        let mut b = Blockchain::new(
            "127.0.0.1".to_string(), "me".to_string(), 1);
        // new() will already call b.create_block(1, "1")
        b.create_block(1, "1".to_string(), Vec::new());
        assert!(!Blockchain::is_chain_valid(&b.chain),
        "chain was valid, even with two identical blocks with prev_hash = 1");

        b = Blockchain::new(
            "127.0.0.2".to_string(), "me2".to_string(), 1);
        let block = b.get_last_block();
        let block_hash = String::from(&Blockchain::hash(&block));
        let proof = Blockchain::proof_of_work(1);
        println!("{}", proof);
        b.create_block(proof, block_hash, Vec::new());
        assert!(Blockchain::is_chain_valid(&b.chain), 
            "chain wasn't valid, even with correct blocks");

    }
}

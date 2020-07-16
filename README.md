# Rust Blockchain exercise

This is a pretty basic blockchain written as an exercise in Rust.

## Usage

```bash
# Let's start two nodes
docker run --rm -p 8089:8088 bc -n Mario
# username: Mario
# address: 32d093b776ca4986b3acadb4fa76000a
docker run --rm -p 8088:8088 bc -n Luigi
# username: Luigi
# address: 3bfbea4302a7411a9bce8c62a9321aba

# you can also launch the server from source with
cargo run -- -n Bowser

# let's get the blockchain from one node:
curl http://localhost:8088/blockchain
```

result

```json
 [
  {
    "index": 0,
    "time": "1684797684",
    "proof": 1,
    "previous_hash": "0"
  }
 ]
```

Let's notify the nodes of their presence:

```bash
curl -v -X POST  -H "Content-Type: application/json" \
  -d '["http://172.17.0.2:8088"]' http://localhost:8088/nodes
# ["http://0.0.0.0:8088/","http://127.0.0.1:8089/"]
curl -v -X POST  -H "Content-Type: application/json" \
  -d '["http://171.17.0.3:8088"]' http://localhost:8089/nodes
```

Now Mario will mine a block, having a coin as reward

```bash
curl -X POST http://localhost:8089/mine_block
```

result

```json
 {
  "index": 1,
  "time": "1684821099",
  "proof": 910,
  "previous_hash": "e7cc0823c3c0e8e704d54866aa4c37f24dff75077f4eda87e31867a499aeadff41e22c141076d4b934d68fe9af60063822c7de304a0143ca1dd2a89dba5f37fc",
  "transactions": [
    {
      "sender": "a398a0901e2b46c8a78983804feff85b",
      "amount": "1",
      "receiver": "Mario"
    }
  ]
}
```

let's update Luigi's chain

```bash
curl http://127.0.0.1:8088/replace_chain
```

results in

```json
{
  "is_replaced": false,
  "blockchain": [
    {
      "index": 0,
      "time": 1684824955,
      "proof": 1,
      "previous_hash": "0",
      "transactions": []
    },
    {
      "index": 1,
      "time": 1684825033,
      "proof": 910,
      "previous_hash": "cc3fefd813c0c418e7ec32773f51c5a9028f720107dbaf1d39004d4a276c9123cae30660d56a1c1b55b0a3df8a9ad445a6880f3107f65a6f396030a6216a6dbf",                                                         
      "transactions": [
        {
          "sender": "32d093b776ca4986b3acadb4fa76000a",
          "amount": 1,
          "receiver": "Mario"
        }
      ]
    }
  ]
}
```

let's add a transaction: Mario will send two coins to Luigi

```bash
curl -v -X POST  -H "Content-Type: application/json" \
    -d '{"sender": "Mario", "amount": 2, "receiver": "Luigi"}' \
    http://localhost:8088/add_transaction
# 3 (is the index of the block on which the transaction will be recorded)
```

and mine a block that will persist that transaction on the chain:

```bash
curl -X POST http://localhost:8088/mine_block
```

with the expected result:

```json
{
  "index": 3,
  "time": 1684825601,
  "proof": 35041,
  "previous_hash": "7faa90e5db00a2cec2ed938b782367ff16feb29ad2846af74f3590829bb9af239e55570b2631f301058ffcbec0955ae2cf88ea614a8bfbce9b559fc28aff6780",
  "transactions": [
    {
      "sender": "Mario",
      "amount": 2,
      "receiver": "Luigi"
    },
    {
      "sender": "c7d5fa96b2f84ec3b34e8d0745685f0c",
      "amount": 1,
      "receiver": "Luigi"
    }
  ]
}
```

## Future improvements

This is a learning toy project so there aren't many features implemented.

But it would be good that:

- [ ] configurable difficulty for the proof
- [ ] configurable IP address (some magic here?) and (wallet) address
- [ ] other nodes should be notified of new blocks
- [ ] other nodes should be notified of new transactions
- [ ] support to bigger chains (so not just in memory and no full-blockchain responses)
- [ ] more tests
- [ ] building Docker container requires openssl vendored, but it should be just
    a sub-dependency

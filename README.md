# ethscan
## Scans the Ethereum network for USDT ERC-20 token transfer transactions

This is a Rust command line program for scanning the Ethereum blockchain for USDT transfers within a time span and amount span. To run it, first install Rust, e.g. from https://rustup.rs.

Then you can build it with `cargo build --release`. Then you can run it like this:

```
./target/release/ethscan http://localhost:8545 2021-11-20 2021-11-21 100000 1000000000

search filter:

date from: 2021-11-20 00:00:00 UTC
date to: 2021-11-21 00:00:00 UTC
amount from: 100000 USDT
amount to: 1000000000 USDT

time (UTC),amount (USDT),transaction
2021-11-20 06:33:05,482440.00,https://etherscan.io/tx/0xfe69e999c6844a46a6722a090ee3a9cfe6338c2d02e721840f280953d84c5270
2021-11-20 06:33:05,600887.31,https://etherscan.io/tx/0x8b175f228c3e38d2cc28f4ab6b4bf1a6c6156883045e7c986ec47c0fe5f63157
2021-11-20 06:33:05,221350.48,https://etherscan.io/tx/0xc360f4e183576848701193f7a456234a0e4bb849a94c7ea960b740da9088d830
2021-11-20 06:32:34,518533.00,https://etherscan.io/tx/0xfc4475e64b1f20b90d7837ea8b4129e1ed0b0fe9a4f0240338f59eca4b125572
2021-11-20 06:31:30,109976.00,https://etherscan.io/tx/0xba5ff95164e88ccfb6aaf9101ada7750ec51f935e14a421db5793f90753510b7
2021-11-20 06:30:29,214000.00,https://etherscan.io/tx/0x965dba12cd3c08f391e7b6a64d96d9b93ee13944339709995362b377b864fe30
2021-11-20 06:30:09,161111.00,https://etherscan.io/tx/0x849d668938a5306f1c202cb9c8fc342767856d777a8ce4058cf0b67b264b7375
2021-11-20 06:30:02,134655.46,https://etherscan.io/tx/0xd4d528bba884f1c3f89a8bd2ffb7b5dc37b47814bff7c51bed8cad79b306606b
2021-11-20 06:29:20,2650000.00,https://etherscan.io/tx/0xfff84cdd25b480089e601649b93d3b8ca97f4e5e21b7e0d0572a8448a022c499
2021-11-20 06:28:36,100000.00,https://etherscan.io/tx/0x66d52c1fdfb99253143a20b652136ecd758b3282265dbca100180e83f96103be
...
```

It needs a fully synced local installation of the geth node, or you can use a service like https://quiknode.pro as well for the RPC interface, but this can be very slow and expensive, because it has to make potentially millions of API requests, depending on the filter period.

TODO: currently it scans all blocks backwards, starting from the latest block. This could be optimized with binary searching for the start block, if the time span to test is long ago. And it could be enhanced to allow other evaluations as well, for example eth transactions, or other tokens.

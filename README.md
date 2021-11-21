# ethscan

This is a Rust command line program for scanning the Ethereum blockchain for USDT transfers within a time span and amount span.

## prerequisites

It needs a fully synced local installation of the `geth` node for the web3 compatible Rust library. Or you can use a service like https://quiknode.pro for the RPC interface, but this can be very slow and expensive, because it has to make potentially millions of API requests, depending on the filter period.

## how to compile and run the program

First install Rust, e.g. from https://rustup.rs.

You can build the program with `cargo build --release`. Example which lists all transactions of at least USDT 10 million on 20th November 2021:

```
./target/release/ethscan http://localhost:8545 2021-11-20 2021-11-21 1e7 1e42

search filter:

date from: 2021-11-20 00:00:00 UTC
date to: 2021-11-21 00:00:00 UTC
amount from: 10000000 USDT
amount to: 1000000000000000000000000000000000000000000 USDT

time (UTC),amount (USDT),from address,to address,transaction
2021-11-20 21:50:23,50000000.000000,0xed344fa1075499dac4e7eb0b868a1874dcdd36cf,0xb8ac6dec186496ad25ef7fdc7f341d93db928d30,0x32cc61b3fc064fd7e7ceab562755316645b72af158a7f4925cdc4f86210d17f5
2021-11-20 16:02:08,10000000.000000,0x6262998ced04146fa42253a5c0af90ca02dfd2a3,0x46340b20830761efd32832a74d7169b29feb9758,0x1ceb25243b19c0ee13bacf476e29ec8a95a317c49ea8581c1984115bb6544e46
2021-11-20 13:03:42,111000000.000000,0x7119cb953db332a3b60730d5d39e7b1ba5120e86,0x0548f59fee79f8832c299e01dca5c76f034f558e,0x95220146c7ec639d001171b4323bb0acafb59da223272b556c0fc8c940250252
2021-11-20 10:01:23,18010797.000000,0x2dc754a674bb130fca3b0e28e5faa1fe719e6756,0x28c6c06298d514db089934071355e5743bf21d60,0xd18cc0c4fad232ad36799bd36f5df05c9d5b9b3233e3aa4ffda215a45f421486
2021-11-20 09:13:42,10000000.000000,0xad6eaa735d9df3d7696fd03984379dae02ed8862,0xf2103b01cd7957f3a9d9726bbb74c0ccd3f355d3,0xf8fb2666804b859d919a3f5888a1a478697334188e6ffe0f9a4af1d8bfd9ed9a
2021-11-20 07:52:42,15800000.000000,0xcf04a13e8b3cbc7770c2347ba8d6f1a1b6605edc,0xda816e2122a8a39b0926bfa84edd3d42477e9efd,0x95dadd60f1ed11d373795da8d8fafc45acc2abce543195601ac3df2f885a2b6b
2021-11-20 07:44:50,13011811.600000,0x4862733b5fddfd35f35ea8ccf08f5045e57388b3,0x83ca25fc236d7dd31e6e5536f94283e3005dc93f,0x87be78f1c81185dcb00d4db1b2deca75d7c304b361aa2c9fc4b9b997d02490d0
2021-11-20 01:56:32,111000000.000000,0x4862733b5fddfd35f35ea8ccf08f5045e57388b3,0x7119cb953db332a3b60730d5d39e7b1ba5120e86,0x71c663b408ec4e52fe1bb9899e1f0f4b02023c875f34222aed1d87aabefaf0d7
2021-11-20 01:30:33,20000000.000000,0x6262998ced04146fa42253a5c0af90ca02dfd2a3,0x46340b20830761efd32832a74d7169b29feb9758,0x7ded214dd8d12948dbcd95d1cfa2a671ce73cefe581dcbedb64bd80fcb2065c3
2021-11-20 01:09:23,49999976.000000,0x21a31ee1afc51d94c2efccaa2092ad1028285549,0x4862733b5fddfd35f35ea8ccf08f5045e57388b3,0xb0818a17731109a91b7b8bf02d926b5423d626c7ed8ad697308e49efabc984e7
2021-11-20 00:35:36,59999976.000000,0xdfd5293d8e347dfe59e90efd55b2956a1343963d,0x4862733b5fddfd35f35ea8ccf08f5045e57388b3,0x1c0fa7d9233130fd311b24e4aad142fed9002845e8a92b55501410afd24b9479
2021-11-20 00:31:52,29305092.000000,0xdfd5293d8e347dfe59e90efd55b2956a1343963d,0x4862733b5fddfd35f35ea8ccf08f5045e57388b3,0xc574481cf7eab8f51386c1cb746f1bf6a00f1383d35cb9afd898018c1dba53ce
2021-11-20 00:22:18,19000000.000000,0x28c6c06298d514db089934071355e5743bf21d60,0x46340b20830761efd32832a74d7169b29feb9758,0x13201a4b51c534e86ee425ea37985b579aaac1390dffc8d2d68c526c6850e1b0

number of transactions: 13, amount sum: 517127652.60 USDT
```

The output can be redirected to a CSV file for using it with other programs, for example a spreadsheet program or gnuplot.

As you can see in the example, scientific number notation for the amount range is also supported. Another example would be `0 1e42` to get all transactions within the specified time period.

Scanning one day needs about 5 minutes, with a local connection on the same computer to a `geth` node.

## geth setup

First install `geth`, as described here: https://geth.ethereum.org/docs/install-and-build/installing-geth and do a full sync of the blockchain. See the `geth` manual for details, but you can do it like this:

```
geth --syncmode "fast" --cache 1024
```

Depending on your computer and internet connection, this might need a few days, 1 GiB RAM for the cache, and about 500 GiB on your harddisk for storing the blockchain. For best performance, use a SSD for the data directory location (default location depends on the operating system, or can be specified with a command line argument).

For using it from the Rust application, you need to run it with RPC service enabled:

```
geth --cache 1024 --http --http.port 8545 --http.addr localhost --http.api personal,eth,net --http.corsdomain '*'
```

This listens to incoming connections from localhost, so the Rust application needs to run on the same computer. You could also bind it to the address 0.0.0.0 to access it from any computer, but this might be a security problem.

## TODO

It could be enhanced to allow other evaluations as well, for example Ether transactions, or other ETC-20 tokens or NFTs.

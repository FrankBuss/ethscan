use chrono::prelude::*;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use std::{env, process};
use web3::transports::Http;
use web3::types::*;
use web3::Web3;

#[derive(Debug, Copy, Clone)]
struct Filter {
    pub date_from: i64,
    pub date_to: i64,
    pub amount_from: f64,
    pub amount_to: f64,
    pub contract: H160,
}

#[derive(Debug, Copy, Clone)]
struct Transfer {
    pub amount: f64,
    pub tx_hash: H256,
}

fn timestamp_to_utc(timestamp: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn date_to_timestamp(date_string: &String) -> i64 {
    if let Ok(date) = NaiveDate::parse_from_str(date_string, "%Y-%m-%d") {
        date.and_time(NaiveTime::from_hms(0, 0, 0)).timestamp()
    } else {
        exit_with_message(&format!(
            "{} is a wrong date format. Use YYYY-MM-DD, for example 2021-01-21.",
            date_string
        ));
    }
}

fn data_to_usdt(data: &Bytes) -> f64 {
    (U256::from_big_endian(&data.0[16..]).as_u128() as f64) / 1000000.0
}

/// test if a transaction matches the filter, and then return the transfer information
async fn test_transaction(
    web3: &Web3<Http>,
    tx: &Transaction,
    filter: &Filter,
) -> web3::Result<Vec<Transfer>> {
    let mut result = Vec::new();
    if let Some(to) = tx.to {
        if to != filter.contract {
            return Ok(result);
        }
    } else {
        return Ok(result);
    }
    let receipt = web3.eth().transaction_receipt(tx.hash).await?.unwrap();
    for log in receipt.logs {
        let topics = log.topics;
        if log.address != filter.contract {
            continue;
        }
        if topics.len() != 3 {
            continue;
        }
        //let from = H160::from_slice(&topics[1].0[12..]);
        //let to = H160::from_slice(&topics[2].0[12..]);
        let amount = data_to_usdt(&log.data);
        if amount < filter.amount_from || amount > filter.amount_to {
            continue;
        }
        let transfer = Transfer {
            amount: amount,
            tx_hash: tx.hash,
        };
        result.push(transfer);
    }
    Ok(result)
}

/**
Test all transactions of a block, or optionally only the specified transaction, and print the transfer information. Return the number of transactions and amount sum.
*/
async fn test_block(
    web3: &Web3<Http>,
    block: &Block<Transaction>,
    tx_hash: Option<H256>,
    filter: &Filter,
) -> web3::Result<(u64, f64)> {
    let time = block.timestamp.as_u64() as i64;
    let time_string = timestamp_to_utc(time);
    let mut sum: f64 = 0.0;
    let mut count: u64 = 0;
    for tx in &block.transactions {
        if let Some(tx_hash) = tx_hash {
            if tx_hash != tx.hash {
                continue;
            }
        }
        let transfers = test_transaction(web3, tx, filter).await?;
        for transfer in transfers {
            println!(
                "{},{:.2},https://etherscan.io/tx/0x{:x}",
                time_string, transfer.amount, tx.hash,
            );
            count += 1;
            sum += transfer.amount;
        }
    }
    Ok((count, sum))
}

fn exit_with_message(message: &str) -> ! {
    eprintln!();
    eprintln!("Error: {}", message);
    process::exit(1);
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        eprintln!(
            "usage: {} RPC-URL date-from date-to amount-from amount-to",
            args[0]
        );
        eprintln!(
            "example: {} http://localhost:8545 2021-11-01 2021-11-20 1000 1100",
            args[0]
        );
        process::exit(1);
    }

    let url = &args[1];
    let usdt_contract: H160 = "0xdac17f958d2ee523a2206206994597c13d831ec7"
        .parse()
        .unwrap();
    let filter = Filter {
        date_from: date_to_timestamp(&args[2]),
        date_to: date_to_timestamp(&args[3]),
        amount_from: args[4].parse::<f64>().unwrap(),
        amount_to: args[5].parse::<f64>().unwrap(),
        contract: usdt_contract,
    };

    println!("search filter:");
    println!("");
    println!("date from: {} UTC", timestamp_to_utc(filter.date_from));
    println!("date to: {} UTC", timestamp_to_utc(filter.date_to));
    println!("amount from: {:2} USDT", filter.amount_from);
    println!("amount to: {:2} USDT", filter.amount_to);
    println!("");

    // open RPC connection
    let transport = web3::transports::Http::new(url)?;
    let web3 = web3::Web3::new(transport);

    // search all blocks, starting from the latest
    let mut block_id = BlockId::Number(BlockNumber::Latest);
    println!("time (UTC),amount (USDT),transaction");
    let mut count_all: u64 = 0;
    let mut sum_all: f64 = 0.0;
    loop {
        let block = web3.eth().block_with_txs(block_id).await?.unwrap();

        let block_number = block.number.unwrap().as_u64();
        if block_number == 0 {
            exit_with_message("Unexpected block number 0. Is geth fully synced?");
        }

        // stop at genesis block
        if block_number == 1 {
            break;
        }

        // previous block
        block_id = BlockId::Number((block_number - 1).into());

        // test current block, if in filtered time span
        let time = block.timestamp.as_u64() as i64;
        if time < filter.date_from {
            break;
        }
        if time < filter.date_from || time > filter.date_to {
            continue;
        }
        let (count, sum) = test_block(&web3, &block, None, &filter).await?;
        count_all += count;
        sum_all += sum;
    }
    println!("");
    println!(
        "number of transactions: {}, amount sum: {:.2} USDT",
        count_all, sum_all
    );
    println!("");

    Ok(())
}

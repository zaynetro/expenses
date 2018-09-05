use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use account::{Account, Transaction};

pub fn parse(out: &mut io::Write, file_path: &str) -> io::Result<Account> {
    writeln!(out, "Reading file {}", file_path)?;

    let f = File::open(file_path)?;
    let reader = BufReader::new(f);

    let mut account = Account {
        number: "Unknown".into(),
        transactions: vec![],
    };

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();
        if i == 0 {
            // Read account number
            account.number = parts[1].into();
        } else if i > 3 && parts.len() >= 13 {
            // Read transaction
            let entry_date: String = parts[0].into();
            let month: String = entry_date.chars().skip(3).collect(); // Keep year and month only
            let transaction = Transaction {
                entry_date,
                month,
                value_date: parts[1].into(),
                payment_date: parts[2].into(),
                amount: parts[3].replace(",", ".").parse().unwrap(),
                beneficiary: parts[4].into(),
                account_number: parts[5].into(),
                bic: parts[6].into(),
                transaction: parts[7].into(),
                reference_number: parts[8].into(),
                originator_reference: parts[9].into(),
                message: parts[10].into(),
                card_number: parts[11].into(),
                receipt: parts[12].into(),
            };
            account.transactions.push(transaction);
        }
    }

    Ok(account)
}

use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

use account::{Account, Transaction};
use style::{bold, underline};

#[derive(Debug)]
struct RecurrentPayment<'a> {
    key: &'a str,
    total_spent: f32,
    transactions: &'a Vec<&'a Transaction>,
}

impl<'a> Ord for RecurrentPayment<'a> {
    fn cmp(&self, other: &RecurrentPayment) -> Ordering {
        self.total_spent
            .partial_cmp(&other.total_spent)
            .unwrap_or(Ordering::Equal)
            .then(self.key.cmp(&other.key))
    }
}

impl<'a> PartialOrd for RecurrentPayment<'a> {
    fn partial_cmp(&self, other: &RecurrentPayment) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for RecurrentPayment<'a> {
    fn eq(&self, other: &RecurrentPayment) -> bool {
        self.key == other.key && self.total_spent.eq(&other.total_spent)
    }
}

impl<'a> Eq for RecurrentPayment<'a> {}

pub fn print(out: &mut io::Write, account: &Account) -> io::Result<()> {
    writeln!(out, "{}", underline("Recurrent payments:"))?;
    writeln!(out)?;

    let mut grouped: HashMap<String, Vec<&Transaction>> = HashMap::new();

    for t in &account.transactions {
        if t.amount > 0.0 {
            // We want to keep only expenses
            continue;
        }

        let key = if t.beneficiary.is_empty() {
            t.transaction.clone()
        } else {
            t.beneficiary.clone()
        };

        grouped.entry(key).or_insert(vec![]).push(t);
    }

    let mut recurrent = vec![];
    let mut max_key_len = 0;

    for (key, transactions) in grouped.iter() {
        if transactions.len() < 2 {
            continue;
        }

        let total_spent = transactions.iter().fold(0.0, |acc, t| acc + t.amount);
        recurrent.push(RecurrentPayment {
            key: &key,
            total_spent,
            transactions: transactions,
        });

        let key_len = key.chars().count() + 2;
        if key_len > max_key_len {
            max_key_len = key_len;
        }
    }

    recurrent.sort();

    for r in recurrent {
        let space = max_key_len - r.key.chars().count();
        writeln!(
            out,
            "    {}: {:space$} {}",
            bold(r.key),
            "",
            underline(format!("Total: {:.2}", -r.total_spent)),
            space = space
        )?;

        for t in r.transactions {
            t.print_short(out)?;
        }
    }

    Ok(())
}

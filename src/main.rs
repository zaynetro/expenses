use std::collections::HashSet;
use std::env;
use std::io;
use std::io::Write;

mod account;
mod parse;
mod recurrent;
mod style;

use account::{Account, Transaction};
use style::{bold, underline};

const TOP_EXPENSES: usize = 10;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut args = env::args();
    let name = args.next().unwrap();
    let file_path = match args.next() {
        Some(file_path) => file_path,
        None => {
            writeln!(&mut handle, "Usage {} <file-path>", name)?;
            return Ok(());
        }
    };

    expenses(&mut handle, file_path)
}

fn expenses(out: &mut io::Write, file_path: String) -> io::Result<()> {
    let account = parse::parse(out, file_path)?;

    print_section_separator(out)?;

    // Account details section
    print_account_details(out, &account)?;
    print_section_separator(out)?;

    // Account summary section
    print_summary_account(out, &account)?;
    print_section_separator(out)?;

    // Months section
    print_months(out, &account)?;
    print_section_separator(out)?;

    // Recurrent transations section
    recurrent::print(out, &account)?;

    Ok(())
}

fn print_section_separator(out: &mut io::Write) -> io::Result<()> {
    writeln!(out)?;
    writeln!(out)
}

fn print_account_details(out: &mut io::Write, account: &Account) -> io::Result<()> {
    writeln!(out, "     Account: {}", account.number)?;
    writeln!(out, "Transactions: {}", account.transactions.len())?;
    let t = &account.transactions;
    match (t.first(), t.last()) {
        (Some(first), Some(last)) => writeln!(
            out,
            "      Period: {} - {}",
            first.entry_date, last.entry_date
        ),
        _ => Ok(()),
    }
}

fn print_summary_account(out: &mut io::Write, account: &Account) -> io::Result<()> {
    writeln!(out, "{}", underline("Summary:"))?;
    writeln!(out)?;

    let income = account.sum(|t| t.amount > 0.0);
    let expenses = account.sum(|t| t.amount < 0.0).abs();
    let profit = income - expenses;

    print_summary(out, income, expenses, profit)
}

fn print_summary(out: &mut io::Write, income: f32, expenses: f32, profit: f32) -> io::Result<()> {
    writeln!(out, "      Income: {:+.2}", income)?;
    writeln!(out, "    Expenses: -{:.2}", expenses)?;
    writeln!(
        out,
        "      {}",
        underline(format!("Profit: {:+.2}", profit))
    )
}

fn print_months(out: &mut io::Write, account: &Account) -> io::Result<()> {
    writeln!(out, "{}", underline("Months:"))?;

    let mut months = HashSet::new();
    for t in &account.transactions {
        months.insert(&t.month);
    }

    let mut months: Vec<&&String> = months.iter().collect();
    months.sort();

    for month in months.iter().rev() {
        writeln!(out)?;
        writeln!(out, "    {}", bold(format!("{}:", month)))?;

        let mut month_transactions: Vec<&Transaction> = account
            .transactions
            .iter()
            .filter(|t| t.month == ***month)
            .collect();

        month_transactions.sort();

        // Print income transactions
        let mut income = 0.0;
        for t in month_transactions.iter().filter(|t| t.amount > 0.0).rev() {
            t.print(out)?;
            income += t.amount;
        }

        // There was income if there were some transactions
        if income > 0.0 {
            writeln!(out)?;
        }

        // Print expenses
        let mut expenses = 0.0;
        for (i, t) in month_transactions
            .iter()
            .filter(|t| t.amount < 0.0)
            .enumerate()
        {
            if i < TOP_EXPENSES {
                // Print only first X
                t.print(out)?;
            }

            expenses += t.amount.abs();
        }

        let profit = income - expenses;

        writeln!(out)?;
        print_summary(out, income, expenses, profit)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testing idea based on this answer: https://stackoverflow.com/a/28370712/2866570

    #[test]
    fn empty_file() {
        let mut output = vec![];
        let res = expenses(&mut output, "./data/empty_transactions.txt".into());
        assert!(res.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");

        let correct = vec![
            "Reading file ./data/empty_transactions.txt",
            "",
            "",
            "     Account: FI1234567890123456",
            "Transactions: 0",
            "",
            "",
            "<u>Summary:</u>",
            "",
            "      Income: +0.00",
            "    Expenses: -0.00",
            "      <u>Profit: +0.00</u>",
            "",
            "",
            "<u>Months:</u>",
            "",
            "",
            "<u>Recurrent payments:</u>",
            "",
        ];

        assert_eq!(correct.len(), output.lines().count());

        for (line, correct) in output.lines().zip(correct) {
            assert_eq!(correct, line);
        }
    }

    #[test]
    fn one_month_one_account() {
        let mut output = vec![];
        let res = expenses(
            &mut output,
            "./data/nordea_one_month_one_account_transactions.txt".into(),
        );
        assert!(res.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");

        let correct = vec![
            "Reading file ./data/nordea_one_month_one_account_transactions.txt",
            "",
            "",
            "     Account: FI1234567890123456",
            "Transactions: 6",
            "      Period: 02.05.2018 - 23.05.2018",
            "",
            "",
            "<u>Summary:</u>",
            "",
            "      Income: +200.00",
            "    Expenses: -21.26",
            "      <u>Profit: +178.74</u>",
            "",
            "",
            "<u>Months:</u>",
            "",
            "    <b>05.2018:</b>",
            "        14.05.2018: +200.00 (Employer - Deposit HELSINKI)",
            "",
            "        02.05.2018: -8.46 (TWILIO - Card purchase USD          10,01 8778894546 KURSSI: 1,1832)",
            "        03.05.2018: -3.80 (RTE Kahvilat Oy - Card purchase HELSINKI)",
            "        23.05.2018: -3.80 (RTE Kahvilat Oy - Card purchase HELSINKI)",
            "        02.05.2018: -2.60 (Iso Tiger Oy - Card purchase HELSINKI)",
            "        12.05.2018: -2.60 (Iso Tiger Oy - Card purchase HELSINKI)",
            "",
            "      Income: +200.00",
            "    Expenses: -21.26",
            "      <u>Profit: +178.74</u>",
            "",
            "",
            "<u>Recurrent payments:</u>",
            "",
            "    <b>RTE Kahvilat Oy</b>:    <u>Total: 7.60</u>",
            "        03.05.2018: 3.80",
            "        23.05.2018: 3.80",
            "    <b>Iso Tiger Oy</b>:       <u>Total: 5.20</u>",
            "        02.05.2018: 2.60",
            "        12.05.2018: 2.60",
        ];

        assert_eq!(correct.len(), output.lines().count());

        for (line, correct) in output.lines().zip(correct) {
            assert_eq!(correct, line);
        }
    }

    // TODO: test incorrect transactions file
    // TODO: test multi month one account
    // TODO: test multi month multi account
}

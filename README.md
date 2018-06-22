# Expenses

Display Nordea expenses in the terminal. Surprisingly, Nordea doesn't
print the summary of expenses.

Other bank formats are welcome.


## Install

1. Clone the repo `git clone git@github.com:zaynetro/expenses.git`
1. Open directory `cd expenses`
1. Run with [cargo](https://github.com/rust-lang/cargo/)
   `cargo run -- ./data/nordea_one_month_one_account_transactions.txt`


## Banks

### Nordea

1. Go to Netbank
1. Choose the account you want transactions for
1. Generate transactions txt file under "Transaction list" section


## Example

Sample output:

```
Reading file ./data/nordea_one_month_one_account_transactions.txt


     Account: FI1234567890123456
Transactions: 6
      Period: 02.05.2018 - 23.05.2018


Summary:

      Income:  +200.00
    Expenses: -21.26
      Profit: +178.74


Months:

    05.2018:
        14.05.2018: +200.00 (Employer - Deposit HELSINKI)

        02.05.2018: -8.46 (TWILIO - Card purchase USD          10,01 8778894546 KURSSI: 1,1832)
        03.05.2018: -3.80 (RTE Kahvilat Oy - Card purchase HELSINKI)
        23.05.2018: -3.80 (RTE Kahvilat Oy - Card purchase HELSINKI)
        02.05.2018: -2.60 (Iso Tiger Oy - Card purchase HELSINKI)
        12.05.2018: -2.60 (Iso Tiger Oy - Card purchase HELSINKI)

      Income:  +200.00
    Expenses: -21.26
      Profit: +178.74


Recurrent payments:

    RTE Kahvilat Oy:    Total: 7.60
        03.05.2018: 3.80
        23.05.2018: 3.80
    Iso Tiger Oy:       Total: 5.20
        02.05.2018: 2.60
        12.05.2018: 2.60
```

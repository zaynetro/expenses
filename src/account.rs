use std::cmp::Ordering;
use std::io;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct Account {
    pub number: String,
    pub transactions: Vec<Transaction>,
}

impl Account {
    /// Sums transaction amounts using predicate
    pub fn sum<P>(&self, predicate: P) -> f32
    where
        P: Fn(&Transaction) -> bool,
    {
        self.transactions
            .iter()
            .filter(|t| predicate(t))
            .fold(0.0, |acc, t| acc + t.amount)
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub entry_date: String,
    pub month: String,
    pub value_date: String,
    pub payment_date: String,
    pub amount: f32,
    pub beneficiary: String,
    pub account_number: String,
    pub bic: String,
    pub transaction: String,
    pub reference_number: String,
    pub originator_reference: String,
    pub message: String,
    pub card_number: String,
    pub receipt: String,
}

impl Transaction {
    pub fn print(&self, out: &mut io::Write) -> io::Result<()> {
        writeln!(
            out,
            "        {}: {:+.2} ({})",
            self.entry_date,
            self.amount,
            self.message().trim(),
        )
    }

    pub fn print_short(&self, out: &mut io::Write) -> io::Result<()> {
        writeln!(out, "        {}: {:.2}", self.entry_date, -self.amount)
    }

    pub fn message(&self) -> String {
        let beneficiary = if self.beneficiary.is_empty() {
            "".into()
        } else {
            format!("{} -", self.beneficiary)
        };
        format!("{} {} {}", beneficiary, self.transaction, self.message)
    }
}

impl Ord for Transaction {
    fn cmp(&self, other: &Transaction) -> Ordering {
        self.amount
            .partial_cmp(&other.amount)
            .unwrap_or(Ordering::Equal)
            .then(self.entry_date.cmp(&other.entry_date))
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Transaction) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Transaction) -> bool {
        self.entry_date == other.entry_date && self.amount.eq(&other.amount)
    }
}

impl Eq for Transaction {}

pub struct Summary {
    pub income: f32,
    pub expenses: f32,
}

impl Summary {
    pub fn profit(&self) -> f32 {
        self.income - self.expenses
    }
}

impl Default for Summary {
    fn default() -> Self {
        Summary {
            income: 0.0,
            expenses: 0.0,
        }
    }
}

impl AddAssign for Summary {
    fn add_assign(&mut self, other: Self) {
        self.income += other.income;
        self.expenses += other.expenses;
    }
}

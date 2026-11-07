# personal-budget
A repo to house the project of making a personal budget tool

## tool components

There are three sources of inputs for the tool:

- Account transaction exports and their corresponding config files (planned to help map different account schema to this tool's schema)
- Budget category and account goals for specific time periods
- Simulation / planning configurations to allow time projection

## transaction schema

Transactions have the following fields:

- Date: Date of transaction
- Amount: Positive amounts indicate a deposit into the account, negative amounts indicate a withdrawl.
- Category: A general category to aid in grouping similar expenses
- Description: A specific description for the transaction
- Account: The specific name for an account, which can be the sources or sinks of the budget

As an example, consider a budget with a checking, savings, credit card, and retirement 401k account. 

On a single day, a paycheck was split between the checking, savings, and retirement accounts, with the initial deposit occurring in the checking account. An existing credit card bill was paid on the same day, and a meal was bought at a restaurant. These transactions would appear like this:

| Date       | Amount   | Category | Description      | Account  |
| ---------- | -------- | -------- | ---------------- | -------- |
| 1970-01-01 | +$100.00 | Income   | Paycheck         | Checking |
| 1970-01-01 | -$30.00  | Transfer | Savings Transfer | Checking |
| 1970-01-01 | +$30.00  | Transfer | Savings Transfer | Savings  |
| 1970-01-01 | -$30.00  | Transfer | 401k Transfer    | Checking |
| 1970-01-01 | +$30.00  | Transfer | 401k Transfer    | 401k     |
| 1970-01-01 | -$50.00  | Transfer | CC Payment       | Checking |
| 1970-01-01 | +$50.00  | Transfer | CC Payment       | CC       |

TODO: Would treating every transaction like a transfer and having from and to properties prevent the double listed transactions? The problem is that the different accounts export transactions just from their perspective, so the import data would have the duplicate records of transactions.

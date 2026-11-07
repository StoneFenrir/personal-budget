# personal-budget

A repo to house the project of making a personal budget tool

## tool components

There are three sources of inputs for the tool:

- Account transaction exports and their corresponding config files (planned to help map different account schema to this tool's schema)
- Budget category and account goals for specific time periods
- Simulation / planning configurations to allow time projection

## transaction schema

Internally to the tool all transactions are treated as transfers between two different accounts with the following fields:

- ID: A hash of all other fields to provide a unique transaction ID
- Date: Date of transaction
- Amount: Dollar amount of the transfer, the direction is indicated by the 'FromAccount' & 'ToAccount' fields
- Category: A general category to aid in grouping similar expenses
- Description: A specific description for the transaction
- FromAccount: The source of the transfer
- ToAccount: The destination of the transfer

As an example, consider a budget with a checking, savings, credit card, and retirement 401k account.

On a single day, a paycheck was split between the checking, savings, and retirement accounts, with the initial deposit occurring in the checking account. An existing credit card bill was paid on the same day, and a meal was bought at a restaurant. These transactions would appear like this:

| Date       | Amount  | Category | Description      | FromAccount | ToAccount | ID (md5)                         |
| ---------- | ------- | -------- | ---------------- | ----------- | --------- | -------------------------------- |
| 1970-01-01 | $100.00 | Income   | Paycheck         | Workplace   | Checking  | 1fe444905da259902b9e1dffcfdb9b44 |
| 1970-01-01 | $30.00  | Transfer | Savings Transfer | Checking    | Savings   | d8ca37971bf85562f035ffb98872bc7d |
| 1970-01-01 | $30.00  | Transfer | 401k Transfer    | Checking    | 401k      | 0e74b20a4bf79362147de4bf413fda78 |
| 1970-01-01 | $50.00  | Transfer | CC Payment       | Checking    | CC        | fc62efe1bf8233dde2fbfed151f7cab6 |

NOTE: Exporting account transactions from their respective institutions (bank, credit card company) will show duplicate transactions, a transfer from one account to another will show up in both places, potentially with different dates. It is also possible for transfers to have fees, where not all money withdrawn from one account is transferred to another. Therefore it is probably required to have all relevant account exports before transactions can be imported to the schema shown above.

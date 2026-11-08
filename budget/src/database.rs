#[cfg(feature="server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("budget.db").expect("Failed to open database");

        conn.execute_batch("CREATE TABLE IF NOT EXISTS transactions (
            id STRING PRIMARY KEY,
            date TEXT,
            amount REAL,
            category TEXT,
            description TEXT,
            from_account TEXT,
            to_account TEXT
        );",).unwrap();
        conn
    }
}
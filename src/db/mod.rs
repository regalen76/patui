pub mod known_networks;

use libsql::{Builder, Connection, Database, OpenFlags};

pub async fn open_database() -> libsql::Result<Database> {
    Builder::new_local("pui.db")
        .flags(OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE)
        .build()
        .await
}

pub async fn migrate(conn: &Connection) -> libsql::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS known_networks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            host TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )
    .await?;

    Ok(())
}

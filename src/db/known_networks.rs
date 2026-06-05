use libsql::Connection;

#[derive(Debug)]
pub struct KnownNetwork {
    pub name: String,
    pub host: String,
}

pub async fn get_all(conn: &Connection) -> libsql::Result<Vec<KnownNetwork>> {
    let mut rows = conn
        .query(
            "SELECT name, host FROM known_networks ORDER BY created_at DESC, id DESC",
            (),
        )
        .await?;
    let mut networks = Vec::new();

    while let Some(row) = rows.next().await? {
        networks.push(KnownNetwork {
            name: row.get(0)?,
            host: row.get(1)?,
        });
    }

    Ok(networks)
}

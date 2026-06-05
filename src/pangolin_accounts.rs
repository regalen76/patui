use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct PangolinAccount {
    pub user_id: String,
    pub email: String,
    pub host: String,
    pub org_id: String,
    pub active: bool,
}

#[derive(Debug, Deserialize)]
struct AccountStore {
    #[serde(rename = "activeUserId", default)]
    active_user_id: String,
    #[serde(default)]
    accounts: HashMap<String, StoredAccount>,
}

#[derive(Debug, Deserialize)]
struct StoredAccount {
    #[serde(rename = "userId", default)]
    user_id: String,
    #[serde(default)]
    host: String,
    #[serde(default)]
    email: String,
    #[serde(rename = "sessionToken", default)]
    session_token: String,
    #[serde(rename = "orgId", default)]
    org_id: String,
}

pub fn load_accounts() -> Result<Vec<PangolinAccount>, String> {
    let path = accounts_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let bytes = fs::read(&path).map_err(|err| format!("read {}: {err}", path.display()))?;
    let store: AccountStore =
        serde_json::from_slice(&bytes).map_err(|err| format!("parse {}: {err}", path.display()))?;

    let mut accounts = store
        .accounts
        .into_values()
        .filter(|account| !account.session_token.is_empty())
        .map(|account| {
            let user_id = account.user_id;
            let active = user_id == store.active_user_id;
            PangolinAccount {
                user_id,
                email: account.email,
                host: account.host,
                org_id: account.org_id,
                active,
            }
        })
        .collect::<Vec<_>>();

    accounts.sort_by(|left, right| {
        right
            .active
            .cmp(&left.active)
            .then_with(|| left.email.cmp(&right.email))
            .then_with(|| left.host.cmp(&right.host))
    });

    Ok(accounts)
}

fn accounts_path() -> Result<PathBuf, String> {
    let home = env::var_os("HOME").ok_or_else(|| String::from("HOME is not set"))?;
    Ok(PathBuf::from(home)
        .join(".config")
        .join("pangolin")
        .join("accounts.json"))
}

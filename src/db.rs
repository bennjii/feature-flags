use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use rusqlite::{params, types::Value, Connection};
use rusqlite_from_row::FromRow;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::error::FeatureFlagError;

pub type DBLite = Arc<Mutex<Connection>>;
pub type DBLocal = Rc<Connection>;

#[derive(Debug, Serialize)]
pub enum FlagDataValue {
    BooleanValue(bool),
    StringValue(String),
    IntegerValue(i32),
    /// JSON encoded string
    CustomValue(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "lowercase")]
pub enum FlagDataType {
    Boolean(bool),
    String(String),
    Integer(i32),
    Custom(String),
}

#[derive(Debug, Serialize, FromRow)]
pub struct FlagWithID {
    pub id: i32,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct FlagWithIDReturn {
    pub id: i32,
    pub name: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Flag {
    pub name: String,
    pub value: FlagDataType,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct FlagValue {
    pub value: FlagDataType,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct KeyOnly {
    pub key: String,
}

pub fn get_db() -> Connection {
    let path = Path::new("instance").join("flag.db");

    Connection::open(path).expect("Unable to find the db")
}

pub fn get_db_rc() -> DBLocal {
    Rc::new(get_db())
}

pub fn get_db_server() -> DBLite {
    let conn = get_db();
    Arc::new(Mutex::new(conn))
}

pub fn initialize_db(conn: DBLocal) -> Result<(), FeatureFlagError> {
    conn.execute_batch(
        "DROP TABLE IF EXISTS flags;

        CREATE TABLE flags (
            id    INTEGER UNIQUE,
            name  TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL,
            PRIMARY KEY(id)
        );",
    )?;

    Ok(())
}

pub async fn initialize_db_arc(conn_mutex: DBLite) -> Result<(), FeatureFlagError> {
    let conn = conn_mutex.lock().await;
    conn.execute_batch(
        "DROP TABLE IF EXISTS flags;

        CREATE TABLE flags (
            id    INTEGER UNIQUE,
            name  TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL,
            PRIMARY KEY(id)
        );",
    )?;

    Ok(())
}

pub fn get_flag_by_name(conn: DBLocal, name: String) -> Result<FlagWithID, FeatureFlagError> {
    let result = conn.query_row(
        "SELECT id, name, value FROM flags WHERE name = ?",
        params![name],
        |row| {
            Ok(FlagWithID {
                id: row.get(0)?,
                name: row.get(1)?,
                value: row.get(2)?,
            })
        },
    )?;

    Ok(result)
}

pub fn get_all_flags(conn: DBLocal) -> Result<Vec<FlagWithID>, FeatureFlagError> {
    let mut stmt = conn.prepare("SELECT id, name, value FROM flags")?;

    let rows = stmt.query_map([], |row| {
        Ok(FlagWithID {
            id: row.get(0)?,
            name: row.get(1)?,
            value: row.get(2)?,
        })
    })?;

    // Convert rows to vec of items
    let mut result = vec![];
    for item in rows {
        result.push(item.unwrap())
    }

    Ok(result)
}

pub fn delete_flag_by_name(conn: DBLocal, name: String) -> Result<usize, FeatureFlagError> {
    let result = conn.execute("DELETE FROM flags WHERE name = ?", params![name])?;

    Ok(result)
}

pub fn add_flag(conn: DBLocal, name: String, value: String) -> Result<usize, FeatureFlagError> {
    let result = conn.execute(
        "INSERT INTO flags (name, value) VALUES (?1, ?2)",
        params![name, value],
    )?;

    Ok(result)
}

pub fn update_flag(conn: DBLocal, name: String, value: String) -> Result<usize, FeatureFlagError> {
    let _ = get_flag_by_name(conn.clone(), name.clone())?;

    let result = conn.execute(
        "UPDATE flags SET value = ? WHERE name = ?",
        params![value, name],
    )?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use rusqlite::Connection;

    use super::*;

    fn in_member_db() -> DBLocal {
        let conn = Connection::open_in_memory().unwrap();

        let local_conn = Rc::new(conn);

        initialize_db(local_conn.clone()).unwrap();

        local_conn
    }

    #[test]
    fn test_delete_flag_failure() {
        let conn = in_member_db();

        let result = delete_flag_by_name(conn.clone(), "test".to_string()).unwrap();

        assert_eq!(result, 0)
    }

    #[test]
    fn test_update_flag_error() {
        let conn = in_member_db();

        let result = update_flag(
            conn.clone(),
            "test".to_string(),
            serde_json::to_string(&FlagDataType::Boolean(true)).unwrap(),
        );

        assert_eq!(
            format!("{:?}", result),
            "Err(RusqliteError(QueryReturnedNoRows))"
        )
    }

    #[test]
    fn test_get_flag_by_name_error() {
        let conn = in_member_db();

        let result = get_flag_by_name(conn.clone(), "test".to_string());

        assert_eq!(
            format!("{:?}", result),
            "Err(RusqliteError(QueryReturnedNoRows))"
        )
    }

    #[test]
    fn test_updating_a_flag() {
        let flag_name = "test_updating".to_string();

        let conn = in_member_db();

        // Initialize the flag to True
        let _ = add_flag(
            conn.clone(),
            flag_name.clone(),
            serde_json::to_string(&FlagDataType::Boolean(true)).unwrap(),
        );

        get_flag_by_name(conn.clone(), flag_name.clone()).unwrap();

        // Update the flag value to False
        let _ = update_flag(
            conn.clone(),
            flag_name.clone(),
            serde_json::to_string(&FlagDataType::Boolean(false)).unwrap(),
        )
        .unwrap();

        get_flag_by_name(conn.clone(), flag_name.clone()).unwrap();
    }

    #[test]
    fn test_add_single_flag() {
        let flag_name = "test_flag".to_string();
        let conn = in_member_db();

        let _ = add_flag(
            conn.clone(),
            flag_name.clone(),
            serde_json::to_string(&FlagDataType::Boolean(true)).unwrap(),
        )
        .unwrap();

        let result = get_flag_by_name(conn.clone(), flag_name.clone()).unwrap();

        assert_eq!(result.name, flag_name);
    }

    #[test]
    fn test_delete_flag() {
        let flag_name = "delete_test".to_string();

        let conn = in_member_db();

        let _ = add_flag(
            conn.clone(),
            flag_name.clone(),
            serde_json::to_string(&FlagDataType::Boolean(true)).unwrap(),
        )
        .unwrap();

        // Make sure the flag was added to the DB
        let flags = get_all_flags(conn.clone()).unwrap();
        assert_eq!(1, flags.len());

        // Delete flag
        let _ = delete_flag_by_name(conn.clone(), flag_name.clone()).unwrap();

        let flags = get_all_flags(conn.clone()).unwrap();
        assert_eq!(0, flags.len());
    }

    #[test]
    fn test_get_all_flags() {
        let conn = in_member_db();

        // Case: Zero Flags
        let result = get_all_flags(conn.clone()).unwrap();
        assert_eq!(0, result.len());

        // Case: More than Zero flags
        let flags = vec![
            (
                "test_1".to_string(),
                serde_json::to_string(&FlagDataType::Boolean(false)).unwrap(),
            ),
            (
                "test_2".to_string(),
                serde_json::to_string(&FlagDataType::Boolean(true)).unwrap(),
            ),
            (
                "test_3".to_string(),
                serde_json::to_string(&FlagDataType::Boolean(false)).unwrap(),
            ),
        ];
        let expected_num_of_flags = flags.len();

        for (name, value) in flags {
            let _ = add_flag(conn.clone(), name, value).unwrap();
        }

        let result = get_all_flags(conn.clone()).unwrap();
        assert_eq!(expected_num_of_flags, result.len());
    }
}

use std::io::Write;

use feature_flags::db;

pub fn update_flag(
    conn: db::DBLocal,
    key: String,
    name: String,
    value: String,
    mut writer: impl Write,
) {
    if key != env!("SEC_KEY") {
        return;
    }

    let result = db::update_flag(conn, name, value);

    match result {
        Ok(_) => writer
            .write_all("Successfully updated the db\n".as_bytes())
            .unwrap(),
        Err(err) => writer
            .write_all(format!("Failed to add to the db: {:?}", err).as_bytes())
            .unwrap(),
    }
}

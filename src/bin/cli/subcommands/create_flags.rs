use std::io::Write;

use feature_flags::db::{add_flag, DBLocal};

pub fn create_flag(db: DBLocal, key: String, name: String, value: String, mut writer: impl Write) {
    if key != env!("SEC_KEY") {
        return;
    }

    let result = add_flag(db, name, value);

    match result {
        Ok(_) => writer
            .write_all("Successfully added to the db\n".as_bytes())
            .unwrap(),
        Err(err) => writer
            .write_all(format!("Failed to add to db: {:?}", err).as_bytes())
            .unwrap(),
    }
}

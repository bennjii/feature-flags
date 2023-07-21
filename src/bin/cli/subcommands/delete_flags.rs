use std::io::Write;

use feature_flags::db::{delete_flag_by_name, DBLocal};

pub fn delete_flag(db: DBLocal, key: String, name: String, mut writer: impl Write) {
    if key != env!("SEC_KEY") {
        return;
    }

    let result = delete_flag_by_name(db, name);
    match result {
        Ok(deleted) => {
            writer
                .write_all(format!("{} row deleted\n", deleted).as_bytes())
                .unwrap();
        }
        Err(err) => {
            writer
                .write_all(format!("delete failed: {:?}\n", err).as_bytes())
                .unwrap();
        }
    };
}

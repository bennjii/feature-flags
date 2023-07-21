use std::io::Write;

use feature_flags::db::{get_flag_by_name, DBLocal};

pub fn get_flag(db: DBLocal, name: String, mut writer: impl Write) {
    let result = get_flag_by_name(db, name);
    match result {
        Ok(flag) => writer
            .write_all(format!("Flag -- {}: {}\n", flag.name, flag.value).as_bytes())
            .unwrap(),
        Err(_) => writer.write_all("No Flag Found\n".as_bytes()).unwrap(),
    };
}

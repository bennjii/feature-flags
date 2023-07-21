use std::io::Write;

use feature_flags::db::{get_all_flags, DBLocal};

pub fn all_flags(db: DBLocal, mut writer: impl Write) {
    let rows = get_all_flags(db).expect("Unable to get all flags");
    for flag in rows {
        writer
            .write_all(format!("flag: {}: {}\n", flag.name, flag.value).as_bytes())
            .unwrap();
    }
    writer.write_all("Done\n".as_bytes()).unwrap();
}

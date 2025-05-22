pub mod level_db {
    use crate::{common::{Env, Status}, util::write_string_to_file_sync};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FileType {
        kLogFile,
        kDBLockFile,
        kTableFile,
        kDescriptorFile,
        kCurrentFile,
        kTempFile,
        kInfoLogFile,
    }


    pub fn parse_filename(filename: &String) -> Option<(u64, FileType)> {
        match filename.as_str() {
            "CURRENT" => Some((0, FileType::kCurrentFile)),
            "LOCK" => Some((0, FileType::kDBLockFile)),
            "LOG" | "LOG.old" => Some((0, FileType::kInfoLogFile)),
            _ if filename.starts_with("MANIFEST-") => {
                match filename.trim_start_matches("MANIFEST-").parse::<u64>() {
                    Ok(number) => Some((number, FileType::kDescriptorFile)),
                    Err(_) => None,
                }
            },
            _ => {
                let (number_str, suffix) = filename.split_once('.').unwrap();
                let number = number_str.parse::<u64>();
                let file_type = match suffix {
                    "log" => FileType::kLogFile,
                    "sst" | "ldb" => FileType::kTableFile,
                    "dbtmp" => FileType::kTempFile,
                    _ => return None,
                };
                Some((number.unwrap_or(0), file_type))
            },
        }
    }

    pub fn make_file_name(dbname: &String, number: u64, suffix: &str) -> String {
        format!("{}/{}.{}", dbname, number, suffix)
    }

    pub fn log_file_name(dbname: &String, number: u64) -> String {
        make_file_name(dbname, number, "log")
    }

    pub fn table_file_name(dbname: &String, number: u64) -> String {
        assert!(number > 0);
        make_file_name(dbname, number, "ldb")
    }

    pub fn ssttable_file_name(dbname: &String, number: u64) -> String {
        assert!(number > 0);
        make_file_name(dbname, number, "sst")
    }

    pub fn descriptor_file_name(dbname: &String, number: u64) -> String {
        make_file_name(dbname, number, "MANIFEST")
    }

    pub fn current_file_name(dbname: &String) -> String {
        format!("{}/CURRENT", dbname)
    }

    pub fn lock_file_name(dbname: &String) -> String {
        format!("{}/LOCK", dbname)
    }

    pub fn info_log_file_name(dbname: &String) -> String {
        format!("{}/LOG", dbname)
    }

    pub fn temp_file_name(dbname: &String, number: u64) -> String {
        format!("{}/LOG.{}", dbname, number)
    }

    pub fn info_log_filename(dbname: &String, number: u64) -> String {
        make_file_name(dbname, number, "LOG")
    }

    pub fn old_info_log_filename(dbname: &String) -> String {
        format!("{}/LOG.old", dbname)
    }

    pub fn set_current_file(env: Box<dyn Env>, dbname: &String, number: u64) -> Status {
        let manifest_file_name = descriptor_file_name(dbname, number);
        assert!(manifest_file_name.starts_with("/"));
        let content = manifest_file_name[dbname.len() + 1..].to_string();
        write_string_to_file_sync(env, &manifest_file_name, &content);

        Status::Ok
    }   
} // mod level_db

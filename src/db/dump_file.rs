mod level_db {
    use crate::{
        common::{Env, Slice, Status, WritableFile},
        db::filename::level_db::{parse_filename, FileType},
    };

    fn guess_type(file_name: &String) -> Option<FileType> {
        let base_name = file_name.split('/').last().unwrap().to_string();

        match parse_filename(&base_name) {
            Some((_, file_type)) => Some(file_type),
            None => None,
        }
    }

    fn dump_log_file(env: Box<dyn Env>, file_name: &String, dst: Box<dyn WritableFile>) -> Status {
        todo!("dump_log_file")
    }

    fn dump_table_file(
        env: Box<dyn Env>,
        file_name: &String,
        dst: Box<dyn WritableFile>,
    ) -> Status {
        todo!("dump_table_file")
    }

    fn dump_descriptor_file(
        env: Box<dyn Env>,
        file_name: &String,
        dst: Box<dyn WritableFile>,
    ) -> Status {
        todo!("dump_descriptor_file")
    }

    pub fn dump_file(env: Box<dyn Env>, file_name: &String, dst: Box<dyn WritableFile>) -> Status {
        let file_type = match guess_type(file_name) {
            Some(file_type) => file_type,
            None => {
                return Status::InvalidArgument(Slice::new_with_string(&format!(
                    "{}: Unknown file type",
                    file_name
                )))
            }
        };
        match file_type {
            FileType::kLogFile => dump_log_file(env, file_name, dst),
            FileType::kTableFile => dump_table_file(env, file_name, dst),
            FileType::kDescriptorFile => dump_descriptor_file(env, file_name, dst),
            _ => {
                return Status::InvalidArgument(Slice::new_with_string(&format!(
                    "{}: not a dump-able file",
                    file_name
                )))
            }
        }
    }
}

pub struct FileMetaStruct {
    pub path: String,
    file_size: usize,
}

impl FileMetaStruct {
    pub fn generate_empty() -> FileMetaStruct {
        FileMetaStruct {
            path: String::from("/dev/null"),
            file_size: 0
        }
    }
}
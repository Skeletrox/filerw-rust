pub mod simplewriter {
    use std::collections::VecDeque;
    use std::fs::File;
    use std::io::Write;
    fn write_to_file(path: &String, values: &String) -> bool {
        let file_size = crate::filereader::simplereader::file_name_size(path);
        let mut file = File::create(path).expect("File should have been created.");
        let res = file.write_all(values.as_bytes());
        match res {
            Ok(_) => return true,
            Err(_) => return false
        }
    }

    pub fn write_vector_to_file(path: &String, values: &mut VecDeque<i32>) -> bool {
        let mut returnable = String::new();
        loop {
            let value = values.pop_front();
            match value {
                Some(x) => {
                    returnable += x.to_string().as_str();
                    returnable += ",";
                }
                None => {
                    assert_eq!(values.len(), 0);
                    break;
                }
            }
        }
        // Remove the last comma
        returnable.pop();
        write_to_file(path, &returnable);
        return true;
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_write_file() {
            let mut values: VecDeque<i32> = VecDeque::<i32>::new();
            values.push_back(10);
            values.push_back(12);
            values.push_front(45);
            values.push_front(128);
            let res = write_vector_to_file(&"/tmp/test_file.tst".to_string(), &mut values);
            assert!(res);
        }
    }
}

pub fn just_returns_hello() -> String {
    String::from("Hello")
}
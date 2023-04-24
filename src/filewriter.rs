pub mod simplewriter {
    use std::collections::VecDeque;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::Write;
    fn write_to_file(path: &String, values: &String) -> bool {
        let mut file = File::create(path).expect("File should have been created.");
        let res = file.write_all(values.as_bytes());
        match res {
            Ok(_) => return true,
            Err(_) => return false
        }
    }

    pub fn append_str_to_file(path: &String, values: &String) -> bool {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        match file.write_all(values.as_bytes()) {
            Ok(_) => return true,
            Err(_) => return false
        }
    }

    pub fn write_vector_to_file_bin(path: &String, values: &mut VecDeque<i32>) -> bool {
        let mut returnable = String::new();
        loop {
            let value = values.pop_front();
            match value {
                Some(x) => {
                    let bytearr = x.to_le_bytes();
                    for b in bytearr {
                        returnable += b.to_string().as_str();
                    }
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
        use rand::Rng;
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

        #[test]
        fn test_write_multiple_vals() {
            let num_lines = 5;
            let num_vals = 10;
            let file_name = "/tmp/rmv_test.tst".to_string();
            let mut rng = rand::thread_rng();
            for i in 0..num_lines {
                let mut values: VecDeque<i32> = VecDeque::<i32>::new();
                for i in 0..num_vals {
                    values.push_back(rng.gen_range(0..65536));
                }
                let mut writable_string = values
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>().join(",");
                writable_string.push('\n');
                append_str_to_file(&file_name, &writable_string);
            }
        }
    }
}

pub fn just_returns_hello() -> String {
    String::from("Hello")
}
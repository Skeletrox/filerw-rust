// Global declaration of "local", crate-specific members is okay,
// but this gets a little wonky for external dependencies.
use crate::structs::FileMetaStruct;

// Publicity of modules is not inherited.
pub mod simplereader {
    use std::str::Split;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::collections::VecDeque;
    pub fn read_file(path: &String) -> String {
        // We should also be able to use the "match" case below
        let file = File::open(path).expect("Should have been able to open the file.");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        match buf_reader.read_to_string(&mut contents) {
            Ok(_) => {},
            Err(err) => panic!("Could not parse string: {:?}!", err),
        };
        return contents;
    }

    fn csv_numbers_to_vector(comma_separated: String) -> VecDeque<i32> {
        let mut returnable: VecDeque<i32> = VecDeque::new();
        let values: Split<&str> = comma_separated.split(",");
        for item in values {
            match item.parse::<i32>() {
                Ok(value) => returnable.push_back(value),
                Err(err) => panic!("Parsing Error! {:?}", err),
            };
        }
        return returnable;
    }
    // Explicitly define publicity for each child module/member.
    pub fn file_name_size(path: &String) -> usize {
        path.len()
    }
    // Read file and get a vector of values
    pub fn read_numbers_from_file(path: &String) -> VecDeque<i32> {
        let csv_contents = read_file(path);
        let returned_values = csv_numbers_to_vector(csv_contents);
        return returned_values;
    }

    #[cfg(test)]
    mod tests {
        use std::collections::VecDeque;
        use super::*;
        use crate::filewriter::simplewriter::write_vector_to_file;

        #[test]
        fn test_read_vals() {
            let mut values: VecDeque<i32> = VecDeque::<i32>::new();
            values.push_back(20);
            values.push_back(25);
            values.push_back(12);
            values.push_back(9);
            values.push_back(42);
            let ground_truth = values.clone();
            let path: String = "/tmp/rw_test.tst".to_string();
            write_vector_to_file(&path, &mut values);
            let res = read_numbers_from_file(&path);
            for i in 0..ground_truth.len() {
                assert_eq!(ground_truth.get(i), res.get(i));
            }
        }
    }
}

pub fn create_empty_struct() -> FileMetaStruct {
    return FileMetaStruct::generate_empty();
}
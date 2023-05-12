// Global declaration of "local", crate-specific members is okay,
// but this gets a little wonky for external dependencies.
use crate::structs::FileMetaStruct;

// Publicity of modules is not inherited.
pub mod simplereader {
    use std::str::Split;
    use std::fs::remove_file;
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

    // Read multiple lines from a file to return multiple vectors of values
    pub fn read_multiple_lines_from_file(path: &String) -> Vec<VecDeque<i32>> {
        let file_contents = read_file(path);
        let lines = file_contents.split("\n");
        let mut returnable = Vec::new();
        for line in lines {
            if line.len() > 0 {
                let current_line_vector = csv_numbers_to_vector(line.to_string());
                returnable.push(current_line_vector);
            }
        }
        return returnable;
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

        #[test]
        fn test_read_multiple_vals() {
            use crate::filewriter::simplewriter::append_str_to_file;
            use rand::Rng;
            let num_lines = 5;
            let num_vals = 10;
            let file_name = "/tmp/rmv_test.tst".to_string();
            let mut rng = rand::thread_rng();
            let mut writing_values: Vec<VecDeque<i32>> = Vec::<VecDeque::<i32>>::new();
            for _ in 0..num_lines {
                let mut values: VecDeque<i32> = VecDeque::<i32>::new();
                for _ in 0..num_vals {
                    values.push_back(rng.gen_range(0..65536));
                }
                let mut writable_string = values
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>().join(",");
                writable_string.push('\n');
                append_str_to_file(&file_name, &writable_string);
                writing_values.push(values);
            }
            let read_values = read_multiple_lines_from_file(&file_name);
            assert_eq!(writing_values.len(), read_values.len());
            for i in 0..writing_values.len() {
                let curr_line_wr = &writing_values[i];
                let curr_line_re = &read_values[i];
                assert_eq!((*curr_line_wr).len(), (*curr_line_re).len());
                for j in 0..curr_line_wr.len() {
                    assert_eq!(curr_line_wr[j], curr_line_re[j]);
                }
            }
            match remove_file(file_name) {
                Ok(_) => {},
                Err(err) => panic!("Could not delete test file: {:?}!", err),
            };

        }
    }
}

pub fn create_empty_struct() -> FileMetaStruct {
    return FileMetaStruct::generate_empty();
}
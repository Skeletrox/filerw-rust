pub mod structs;
pub mod filereader;
pub mod filewriter;
pub mod valsorter;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    use crate::filewriter::simplewriter::write_vector_to_file;
    use crate::filereader::simplereader::read_numbers_from_file;
    use crate::valsorter::intsorter::sortvals;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_end_to_end() {
        let mut vals: VecDeque<i32> = VecDeque::<i32>::new();
        vals.push_back(20);
        vals.push_back(35);
        vals.push_back(66);
        vals.push_back(234);
        vals.push_back(-2);
        vals.push_back(35);
        vals.push_back(0);
        vals.push_back(2);
        let ground_truth = vals.clone();
        // generate the file we read from to sort
        let path = "/tmp/e2e_test".to_string();
        let path_sorted = "/tmp/e2e_sorted".to_string();
        write_vector_to_file(&path, &mut vals);
        vals = read_numbers_from_file(&path);
        // Make sure there are no R/W errors
        for i in 0..vals.len() {
            assert_eq!(ground_truth.get(i), vals.get(i));
        }
        let sorted_vals = sortvals(&mut vals);
        // Assert sortedness
        for i in 0..sorted_vals.len() - 1 {
            assert!(sorted_vals.get(i) <= sorted_vals.get(i+1));
        }
        let mut sorted_vals_mut = sorted_vals.clone();
        write_vector_to_file(&path_sorted, &mut sorted_vals_mut);
        sorted_vals_mut = read_numbers_from_file(&path_sorted);
        // Assert sortedness
        for i in 0..sorted_vals_mut.len() - 1 {
            assert!(sorted_vals_mut.get(i) <= sorted_vals_mut.get(i+1));
        }
    }
}

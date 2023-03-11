pub mod intsorter {
    use std::collections::VecDeque;

    pub fn sortvals(vals: &mut VecDeque<i32>) -> VecDeque<i32> {
        let mut vals_as_vec: Vec<i32> = vals.iter().cloned().collect();
        vals_as_vec.sort();
        // I cannot transmute & to &mut as it is undefined behavior and
        // Rust will complain :(. Instead we shall defer mutation to
        // the caller
        return vals_as_vec.iter().cloned().collect::<VecDeque<i32>>();
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_sort_vals() {
            let mut values: VecDeque<i32> = VecDeque::<i32>::new();
            values.push_back(10);
            values.push_back(12);
            values.push_front(45);
            values.push_front(128);
            let res = sortvals(&mut values);
            let len = res.len();
            for i in 0..len-1 {
                assert!(res.get(i) <= res.get(i+1));
            }
        }
    }
}
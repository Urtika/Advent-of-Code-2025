pub mod day2 {
    pub fn add_invalid_ids(id_ranges: &str) -> u32 {
        // split data into individual ranges and iterate through each range
        // adding the sums of invalid IDs together
        unimplemented!();
    }

    pub fn sum_invalid_ids_in_range(range: &str) -> u32 {
        // calculate the invalid IDs in the given range and return their sum
        // cases:
        // odd number digits first ID -> if second ID same number stop, otherwise calculate all inbetween
        // even number digits first ID -> cut number in half, take most sig digits and check double is in range
        // keep adding by one to first digits and check in range until number becomes greater than second one
        
        let mut sum: u32 = 0;
        // parse the number range around '-' into numbers and their length in digits
        let split: usize = range.find("-").unwrap();

        let start: &str = &range[..split];
        let start_num_digits: usize = start.len();
        let start: u32 = start.parse().expect("Given non-number in range");

        let end: &str = &range[split+1..];
        let end_num_digits: usize = end.len();
        let end: u32 = end.parse().expect("Given non-number in range");

        // go through our cases:
        if start_num_digits % 2 == 1 {
            if end_num_digits == start_num_digits {
                return sum
            }
            let half_power: u32 = start_num_digits as u32 / 2;
            let mut tracker: u32 = 10_u32.pow(half_power);
            let mut invalid_id = double_number(tracker);
            while invalid_id <= end {
                sum = sum + invalid_id;
                tracker = tracker + 1;
                invalid_id = double_number(tracker);
            }
            return sum
        }
        let mut tracker = start / 10_u32.pow(start_num_digits as u32 / 2);
        let mut invalid_id: u32 = double_number(tracker);
        while invalid_id <= end {
            if invalid_id >= start {
                sum = sum + invalid_id;
            }
            tracker = tracker + 1;
            invalid_id = double_number(tracker);
            }
        return sum
    }

    pub fn count_digits(num: u32) -> u32 {
        // returns the number of digits in a number
        let mut digits: u32 = 0;
        let mut track: u32 = num;
        while track > 0 {
            track = track / 10;
            digits = digits + 1;
        }

        digits
    }

    pub fn double_number(num: u32) -> u32 {
        // "doubles" the number in a stringlike fashion
        // eg. double_number(123) returns 123123
        let num_digits: u32 = count_digits(num);
        let doubled: u32 = num * 10_u32.pow(num_digits) + num;
        doubled
    }

}

#[cfg(test)]
mod tests {
    use std::result;

    use crate::days::day2::day2::*;

    #[test]
    fn day2_test_double_number_1() {
        let result: u32 = double_number(10);
        assert_eq!(result, 1010);
    }

    #[test]
    fn day2_test_double_number_2() {
        let result: u32 = double_number(123);
        assert_eq!(result, 123123);
    }

    #[test]
    fn day2_test_count_digits_simple() {
        let result: u32 = count_digits(123456789);
        assert_eq!(result, 9);
    }

    #[test]
    fn day2_test_count_digits_zero() {
        let result: u32 = count_digits(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn day2_test_sum_in_range_simple_even() {
        let result: u32 = sum_invalid_ids_in_range("11-87");
        let expected: u32 = 11 + 22 + 33 + 44 + 55 + 66 + 77;
        assert_eq!(result, expected);
    }

    #[test]
    fn day2_test_sum_in_range_simple_odd() {
        let result: u32 = sum_invalid_ids_in_range("101-999");
        assert_eq!(result, 0);
    }

    #[test]
    fn day2_test_sum_in_range_overlap_even() {
        let result: u32 = sum_invalid_ids_in_range("90-1011");
        let expected: u32 = 99 + 1010;
        assert_eq!(result, expected);
    }

    #[test]
    fn day2_test_sum_in_range_overlap_odd() {
        let result: u32 = sum_invalid_ids_in_range("1-333");
        let expected: u32 = 11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99;
        assert_eq!(result, expected);
    }


    #[test]
    fn day2_test_add_invalid_ids() {
        // given example from problem
        let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result: u32 = add_invalid_ids(input);
        assert_eq!(result, 1227775554);
    }

}
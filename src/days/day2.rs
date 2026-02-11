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
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day2::day2::*;

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
pub mod day1 {
    pub fn calculate_password(instructions: &str) -> u32 {
        // takes a list of instructions separated by newlines (instruction = ^[L|R]d.)
        // and returns the password
        let mut dial_value: u32 = 50;
        let mut password: u32 = 0;
        for instruction in instructions.lines() {
            match parse_instruction(instruction) {
                Some(val) => { dial_value = get_next_dial_value(dial_value, val);
                            if dial_value == 0 {
                                password = password + 1;
                            }},
                _ => continue,
            }
        }
        password
    }

    pub fn parse_instruction(instruction: &str) -> Option<u32> {
        // parses one line of instructions into the value to turn dial by
        // does not return anything if instruction is garbage
        if instruction.starts_with("R") {
            // parse rest as turn right
            let mut turn_val: u32 = instruction[1..].parse::<u32>().ok()?;
            if turn_val > 99 {
                turn_val = turn_val % 100;
            }
            Some(turn_val)
        } else if instruction.starts_with("L") {
            // parse rest as turn left
            let mut turn_val: u32 = instruction[1..].parse::<u32>().ok()?;
            if turn_val > 99 {
                turn_val = turn_val % 100;
            }
            Some(100 - turn_val)
        } else {
            None
        }
    }

    pub fn get_next_dial_value(curr_val: u32, turn_val: u32) -> u32 {
        // takes current dial value and calculates the next one
        (curr_val + turn_val) % 100
    }

    pub fn calculate_password2(instructions: &str) -> u32 {
        // new password calculator that checks every pass through 0 on dial
        let mut dial_value: u32 = 50;
        let mut password: u32 = 0;
        let mut clicks: u32;

        for instruction in instructions.lines() {
            match parse_instruction2(instruction) {
                Some((val, dir)) => { 
                    (dial_value, clicks) = get_next_dial_value2(dial_value, val, dir);
                    //println!("New dial value: {dial_value}, clicks: {clicks}");
                    password = password + clicks;
                    },
                _ => continue,
            }
        }
        password
    }

    pub fn parse_instruction2(instruction: &str) -> Option<(u32, bool)> {
        // parses the instruction (without intermediate modulo of turn value)
        // into the turn value and turn direction (true if right)
        // returns None for garbage instructions
        if instruction.starts_with("R") {
            // parse rest as turn right
            let turn_val: u32 = instruction[1..].parse::<u32>().ok()?;
            Some((turn_val, true))
        } else if instruction.starts_with("L") {
            // parse rest as turn left
            let turn_val: u32 = instruction[1..].parse::<u32>().ok()?;
            Some((turn_val, false))
        } else {
            None
        }
    }

    pub fn get_next_dial_value2(curr_val: u32, turn_val: u32, r_dir: bool) -> (u32, u32) {
        // given turn value and turn direction,
        // returns the new dial value and number of 'clicks' representing passes through 0 on dial
        let mut clicks: u32 = turn_val / 100;
        let turn_rem: i32 = (turn_val % 100).try_into().unwrap();
        let curr_val_signed: i32 = curr_val.try_into().unwrap();

        if r_dir {
            let right_turn: u32 = (curr_val_signed + turn_rem).try_into().unwrap();
            clicks = clicks + (right_turn / 100);
            ((right_turn % 100).try_into().unwrap(), clicks)
        } else {
            let left_turn: i32 = curr_val_signed - turn_rem;
            if left_turn <= 0 {
                if curr_val != 0 {
                    clicks = clicks + 1;
                }
                
                (((100 + left_turn) % 100).try_into().unwrap(), clicks)
            } else {
                ((curr_val_signed - turn_rem).try_into().unwrap(), clicks)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day1::day1::*;

    #[test]
    fn day1_test_next_dial_simple() {
        // most basic test
        let result = get_next_dial_value(11, 8);
        assert_eq!(result, 19);
    }

    #[test]
    fn day1_test_next_dial_zero() {
        // test when next dial hits 0
        let result = get_next_dial_value(19, 81);
        assert_eq!(result, 0);
    }

    #[test]
    fn day1_test_next_dial_wrap() {
        // test next dial when turn passes 0
        let result = get_next_dial_value(50, 51);
        assert_eq!(result, 1);
    }

    #[test]
    fn day1_test_parse_r_inst1() {
        // test parse R instruction
        let result = parse_instruction("R4");
        assert_eq!(result, Some(4));
    }

    #[test]
    fn day1_test_parse_r_inst2() {
        // test parse R instruction
        let result = parse_instruction("R14");
        assert_eq!(result, Some(14));
    }

    #[test]
    fn day1_test_parse_r_inst3() {
        // test parse R instruction with overflow
        let result = parse_instruction("R150");
        assert_eq!(result, Some(50));
    }

    #[test]
    fn day1_test_parse_r_garbage() {
        // test parse R instruction when turn value is garbage
        let result = parse_instruction("R7j4");
        assert_eq!(result, None);
    }

    #[test]
    fn day1_test_parse_l_inst1() {
        // test parse L instruction
        let result = parse_instruction("L4");
        assert_eq!(result, Some(96));
    }

    #[test]
    fn day1_test_parse_l_inst2() {
        // test parse L instruction
        let result = parse_instruction("L14");
        assert_eq!(result, Some(86));
    }

    #[test]
    fn day1_test_parse_l_inst3() {
        // test parse L instruction
        let result = parse_instruction("L150");
        assert_eq!(result, Some(50));
    }

    #[test]
    fn day1_test_parse_l_garbage() {
        // test parse L instruction when turn value is garbage
        let result = parse_instruction("L9o4");
        assert_eq!(result, None);
    }

    #[test]
    fn day1_test_parse_garbage_inst() {
        // test parse instruction when instruction is garbage
        let result = parse_instruction(" ");
        assert_eq!(result, None);
    }

    #[test]
    fn day1_test_calc_pwd() {
        // test to calculate password
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let result = calculate_password(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn day1_test_next_dial2_simple_l() {
        // most basic test when turn left
        let (result, clicks) = get_next_dial_value2(11, 8, false);
        assert_eq!(result, 3);
        assert_eq!(clicks, 0);
    }

    #[test]
    fn day1_test_next_dial2_simple_r() {
        // most basic test when turn right
        let (result, clicks) = get_next_dial_value2(11, 8, true);
        assert_eq!(result, 19);
        assert_eq!(clicks, 0);
    }

    #[test]
    fn day1_test_next_dial2_zero_r() {
        // test when next dial hits 0
        let (result, clicks) = get_next_dial_value2(50, 150, true);
        assert_eq!(result, 0);
        assert_eq!(clicks, 2);
    }

        #[test]
    fn day1_test_next_dial2_zero_l() {
        // test when next dial hits 0
        let (result, clicks) = get_next_dial_value2(50, 150, false);
        assert_eq!(result, 0);
        assert_eq!(clicks, 2);
    }

    #[test]
    fn day1_test_next_dial2_wrap_l() {
        // test next dial when turn passes 0
        let (result, clicks) = get_next_dial_value2(50, 51, false);
        assert_eq!(result, 99);
        assert_eq!(clicks, 1);
    }

    #[test]
    fn day1_test_next_dial2_wrap_r() {
        // test next dial when turn passes 0
        let (result, clicks) = get_next_dial_value2(50, 51, true);
        assert_eq!(result, 1);
        assert_eq!(clicks, 1);
    }

    #[test]
    fn day1_test_next_dial2_wraps_l() {
        // test next dial when turn passes 0 multiple times
        let (result, clicks) = get_next_dial_value2(20, 730, false);
        assert_eq!(result, 90);
        assert_eq!(clicks, 8);
    }

    #[test]
    fn day1_test_next_dial2_wraps_r() {
        // test next dial when turn passes 0 multiple times
        let (result, clicks) = get_next_dial_value2(20, 730, true);
        assert_eq!(result, 50);
        assert_eq!(clicks, 7);
    }

    #[test]
    fn day1_test_parse2_r_inst1() {
        // test parse R instruction
        let result = parse_instruction2("R4");
        assert_eq!(result, Some((4, true)));
    }

    #[test]
    fn day1_test_parse2_r_inst2() {
        // test parse R instruction
        let result = parse_instruction2("R14");
        assert_eq!(result, Some((14, true)));
    }

    #[test]
    fn day1_test_parse2_r_inst3() {
        // test parse R instruction with overflow
        let result = parse_instruction2("R150");
        assert_eq!(result, Some((150, true)));
    }

    #[test]
    fn day1_test_parse2_r_garbage() {
        // test parse R instruction when turn value is garbage
        let result = parse_instruction2("R7j4");
        assert_eq!(result, None);
    }

    #[test]
    fn day1_test_parse2_l_inst1() {
        // test parse L instruction
        let result = parse_instruction2("L4");
        assert_eq!(result, Some((4, false)));
    }

    #[test]
    fn day1_test_parse2_l_inst2() {
        // test parse L instruction
        let result = parse_instruction2("L14");
        assert_eq!(result, Some((14, false)));
    }

    #[test]
    fn day1_test_parse2_l_inst3() {
        // test parse L instruction
        let result = parse_instruction2("L150");
        assert_eq!(result, Some((150, false)));
    }

    #[test]
    fn day1_test_parse2_l_garbage() {
        // test parse L instruction when turn value is garbage
        let result = parse_instruction2("L9o4");
        assert_eq!(result, None);
    }

    #[test]
    fn day1_test_parse2_garbage_inst() {
        // test parse instruction when instruction is garbage
        let result = parse_instruction2(" ");
        assert_eq!(result, None);
    }

    #[test]
    fn day1_test_calc_pwd2() {
        // test to calculate password
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let result = calculate_password2(input);
        assert_eq!(result, 6);
    }

}
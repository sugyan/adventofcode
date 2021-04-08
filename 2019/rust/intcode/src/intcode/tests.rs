#[cfg(test)]
use crate::Intcode;

#[test]
fn day02_example_1() {
    {
        let mut computer = Intcode::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        computer.run(Vec::new());
        assert_eq!(
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            computer.program
        );
    }
    {
        let mut computer = Intcode::new(&[1, 0, 0, 0, 99]);
        computer.run(Vec::new());
        assert_eq!(vec![2, 0, 0, 0, 99], computer.program);
    }
    {
        let mut computer = Intcode::new(&[2, 3, 0, 3, 99]);
        computer.run(Vec::new());
        assert_eq!(vec![2, 3, 0, 6, 99], computer.program);
    }
    {
        let mut computer = Intcode::new(&[2, 4, 4, 5, 99, 0]);
        computer.run(Vec::new());
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], computer.program);
    }
    {
        let mut computer = Intcode::new(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        computer.run(Vec::new());
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], computer.program);
    }
}

#[test]
fn day05_example_1() {
    // Outputs whatever it gets as input
    {
        let mut computer = Intcode::new(&[3, 0, 4, 0, 99]);
        let output = computer.run(vec![42]);
        assert_eq!(Some(42), output);
    }
    // 99 is written to address 4
    {
        let mut computer = Intcode::new(&[1002, 4, 3, 4, 33]);
        computer.run(Vec::new());
        assert_eq!(vec![1002, 4, 3, 4, 99], computer.program);
    }
    // 99 is written to address 4 (Integers can be negative)
    {
        let mut computer = Intcode::new(&[1101, 100, -1, 4, 0]);
        computer.run(Vec::new());
        assert_eq!(vec![1101, 100, -1, 4, 99], computer.program);
    }
}

#[test]
fn day05_example_2() {
    // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not)
    for &(input, expected) in &[(7, 0), (8, 1), (9, 0)] {
        let mut computer = Intcode::new(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        let output = computer.run(vec![input]);
        assert_eq!(Some(expected), output);
    }
    // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)
    for &(input, expected) in &[(7, 1), (8, 0)] {
        let mut computer = Intcode::new(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        let output = computer.run(vec![input]);
        assert_eq!(Some(expected), output);
    }
    // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not)
    for &(input, expected) in &[(7, 0), (8, 1), (9, 0)] {
        let mut computer = Intcode::new(&[3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let output = computer.run(vec![input]);
        assert_eq!(Some(expected), output);
    }
    // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)
    for &(input, expected) in &[(7, 1), (8, 0)] {
        let mut computer = Intcode::new(&[3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        let output = computer.run(vec![input]);
        assert_eq!(Some(expected), output);
    }
    // Take an input, then output 0 if the input was zero or 1 if the input was non-zero (using position mode)
    for &(input, expected) in &[(-1, 1), (0, 0), (1, 1)] {
        let mut computer = Intcode::new(&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
        let output = computer.run(vec![input]);
        assert_eq!(Some(expected), output);
    }
    // Take an input, then output 0 if the input was zero or 1 if the input was non-zero (using immediate mode)
    for &(input, expected) in &[(-1, 1), (0, 0), (1, 1)] {
        let mut computer = Intcode::new(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        let output = computer.run(vec![input]);
        assert_eq!(Some(expected), output);
    }
    // Output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8
    for &(input, expected) in &[(6, 999), (7, 999), (8, 1000), (9, 1001), (10, 1001)] {
        let mut computer = Intcode::new(&[
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        let output = computer.run(vec![input]);
        assert_eq!(Some(expected), output);
    }
}

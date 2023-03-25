pub fn is_armstrong_number(num: u32) -> bool {
    let numbers: Vec<u64> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let mut is_armstrong: u64 = 0;
    numbers
        .iter()
        .for_each(|x| is_armstrong += x.pow(numbers.len() as u32));

    if is_armstrong == num as u64 {
        return true;
    }
    false
}

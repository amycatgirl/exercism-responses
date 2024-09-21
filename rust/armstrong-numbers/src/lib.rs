pub fn is_armstrong_number(num: u32) -> bool {
    let string = &num.to_string();
    let number_list = string.chars();
    let size = u32::try_from(string.len());

    let acc_result = number_list.fold(0, |acc, x| {
        acc + (x.to_digit(10).unwrap().pow(size.unwrap()))
    });

    acc_result == num
}

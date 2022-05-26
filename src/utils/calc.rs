pub fn equate(s: String) -> f64 {
    let mut calc_buf = String::new();
    let mut in_parenthesis = false;
    let mut parenthesis_buf = String::new();
    for c in s.chars() {
        if c == '(' {
            in_parenthesis = true;
        }
        else if c == ')' {
            in_parenthesis = false;
            calc_buf += &calc(String::from(&parenthesis_buf)).to_string();
            parenthesis_buf = String::new();
        }
        else if in_parenthesis {
            parenthesis_buf.push(c);
        }
        else {
            calc_buf.push(c);
        }
    }
    return calc(calc_buf);
}

fn calc(inp: String) -> f64 {
    let mut num_buf: String = String::new();
    let mut nums: Vec<f64> = vec![];
    let mut op: char = '\0';
    for c in inp.chars() {
        if c == ' ' {}                          // Skip Whitespaces because they are irrelevant.
        else if "1234567890.".contains(c) {     // Buffer our number so we can later parse int to an f64
            num_buf.push(c)
        }
        else if "+-*/^".contains(c) {
            nums.push(f64parse(num_buf));
            num_buf = String::new();
            if nums.len() % 2 == 0 {
                let num1 = nums.pop().unwrap();
                let num2 = nums.pop().unwrap();
                nums.push(calc_nums(num1, op, num2))
            }
            op = c;
        }
    }
    nums.push(f64parse(num_buf));
    if nums.len() % 2 == 0 {
        let num1 = nums.pop().unwrap();
        let num2 = nums.pop().unwrap();
        nums.push(calc_nums(num1, op, num2))
    }
    return nums[0];
}

fn calc_nums(n1: f64, op: char, n2: f64) -> f64 {
    return match op {
        '+' => n1 + n2,
        '-' => n1 - n2,
        '*' => n1 * n2,
        '/' => n1 / n2,
        '^' => n2.powf(n1),
        _   => 0.0
    }
}

pub fn f64parse(num: String) -> f64 {
    let decimal = num.chars().position(|c| c == '.');
    let dec_point = match decimal {
        None => Option::from(num.len()),
        _ => decimal   
    }.unwrap();
    let mut val = 0.0;
    if dec_point != num.len() {
        for (i, c) in num[dec_point+1..].chars().enumerate() {
            val += char_to_num(c) * ((0.1 as f64).powf((i as f64)+1.0))
        }
    }
    for (i, c) in num[..dec_point].chars().rev().enumerate() {
        val += char_to_num(c) * ((10 as f64).powf(i as f64))
    }
    return val;
}

fn char_to_num(char: char) -> f64 {
    return match char {
        '0' => 0.0,
        '1' => 1.0,
        '2' => 2.0,
        '3' => 3.0,
        '4' => 4.0,
        '5' => 5.0,
        '6' => 6.0,
        '7' => 7.0,
        '8' => 8.0,
        '9' => 9.0,
        _   => 0.0,
    }
}

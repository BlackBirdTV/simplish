use std::io::{stdin,stdout,Write};
mod calc;
pub static mut VARS: Vec<KeyValuePair> = vec![];

pub struct KeyValuePair {
    pub key: String,
    pub value: String
}

pub fn readln(prefix: &str) -> String {
    let mut s=String::new();
    print!("{}", prefix);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;
}

pub fn readtxt(prev_text: String) -> String {
    let mut text = prev_text;
    
    println!("Type :c to clear the file or :s to save and exit");
    print!("{}", text);
    loop {
        let inp = &readln("")[..];
        if inp == ":c" {
            return "".to_owned();
        }
        else if inp == ":s" {
            return text;
        }
        text += inp;
        text += "\n";
    }
}

pub fn error(msg: &str) {
    println!("\x1b[38;5;196m{}\x1b[0m", msg)
}

pub fn expr(exprl: &str) -> String {
    if !exprl.contains(':') {
        error("Invalid action format! Use actions like this: \x1b[3maction\x1b[23m:\x1b[3minput\x1b[23m");
        return "NULL".to_owned();
    }
    let colon_ind = exprl.chars().position(|c| c == ':').unwrap();
    let action = &exprl[..colon_ind].to_owned();
    let expra = &exprl[colon_ind+1..].to_owned();
    let mut parsed_expr = String::new();
    let mut in_expr = false;
    let mut expr_buf = String::new();
    for c in expra.chars() {
        if c == '&' {
            in_expr = !in_expr;
            if !in_expr {
                parsed_expr += &expr(&expr_buf);
            }
        }
        else if in_expr {
            expr_buf.push(c);
        }
        else {
            parsed_expr.push(c);
        }
    }
    return match &action[..] {
        "var"   => get_var(parsed_expr.to_owned()),
        "calc"  => calc::equate(parsed_expr.to_owned()).to_string(),
        "equ"   => equ(parsed_expr.to_owned()).to_string(),
        _       => { error(&format!("Action {} not found", action)); return String::new() }
    }
}

pub fn get_var(name: String) -> String {
    unsafe {
    for v in VARS.iter() {
        if v.key == name {
            return v.value.to_owned();
        }
    }
    error(&format!("Variable {} not found", name));
    return "NULL".to_owned();
    }
}

pub fn parse(inp: String) -> Vec<String> {
    let mut parsed: Vec<String> = vec![];
    let mut in_str: bool = false;
    let mut in_expr: bool = false;
    let mut str_char: char = '"';
    let mut str_buf = String::new();
    let mut expr_buf = String::new();
    for c in inp.chars() {
        if c == '"' || c == '\'' {
            if str_char == c && in_str {
                in_str = false;
            }
            else if in_str {
                str_buf.push(c);
            }
            else {
                in_str = true;
                str_char = c;
            }
        }
        else if c == '%' {
            in_expr = !in_expr;
            if !in_expr {
                str_buf += &expr(&expr_buf);
            }
        }
        else if in_expr {
            expr_buf.push(c);
        }
        else if c == ' ' {
            if in_str {
                str_buf.push(c);
            }
            else {
                parsed.push(str_buf);
                str_buf = String::new();
            }
        }
        else {
            str_buf.push(c);
        }
    }
    if str_buf.len() > 0 {
        parsed.push(str_buf);
    }
    return parsed;
}

pub fn equ(inp: String) -> f64 {
    let mut op = "";
    let mut op_idx = 0;
    for (i, c) in inp.chars().enumerate() {
        if  (c == '=' && inp.chars().nth(i-1).unwrap() == '=') || 
            (c == '=' && inp.chars().nth(i-1).unwrap() == '!') ||
            (c == '=' && inp.chars().nth(i-1).unwrap() == '<') ||
            (c == '=' && inp.chars().nth(i-1).unwrap() == '>') ||
            (c == '<' && inp.chars().nth(i-1).unwrap() == '<') ||
            (c == '>' && inp.chars().nth(i-1).unwrap() == '>')
        {
            op = &inp[i-1..i+1];
            op_idx = i;
        }
    }
    if op_idx < 1 {
        error("Invalid op");
        return 0.0;
    }
    let first = &inp[..op_idx-1];
    let second = &inp[op_idx+1..];
    let n = match op {
        "==" => first == second,
        "!=" => first != second,
        ">=" => calc::f64parse(first.to_owned()) >= calc::f64parse(second.to_owned()),
        "<=" => calc::f64parse(first.to_owned()) <= calc::f64parse(second.to_owned()),
        "<<" => calc::f64parse(first.to_owned()) < calc::f64parse(second.to_owned()),
        ">>" => calc::f64parse(first.to_owned()) > calc::f64parse(second.to_owned()),
        _ => false
    };
    return match n { true => 1.0, false => 0.0 };
}

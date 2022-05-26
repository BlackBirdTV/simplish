use std::path::{Path};
use std::env;
use std::fs;
use colored::{*};
mod utils;

static mut ANSI_ENABLED: bool = true;

fn main() {
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => {
            // ANSI escape codes were successfully enabled, or this is a non-Windows platform.
        }
        Err(_) => {
            utils::error("Your version of Windows is deprecated, so ANSI text coloring will be disabled.");
            unsafe { ANSI_ENABLED = false; }
        }
    }
    t_clear();
    loop {
        match env::current_dir() {
            Ok(path) => {
                unsafe {
                    if ANSI_ENABLED {
                        print!("{}",
                            format!("{}:{}", 
                                format!("{}@{}", 
                                    whoami::username(),
                                    whoami::devicename().replace(" ", "-")
                                ).bright_green(),   
                                format!("{}", path.into_os_string()
                                    .into_string()
                                    .unwrap()
                                    .bright_blue()
                                )
                            ).bold()
                        )
                    }
                    else {
                        print!("{}@{}:{}",
                            whoami::username(),
                            whoami::devicename().replace(" ", "-"),   
                            path.into_os_string()
                                .into_string()
                                .unwrap()
                            )
                    }
                }
            },
            _ => println!("Error: INVALID CURRENT DIRECTORY")
        }
        let inp = utils::parse(utils::readln("¢ "));
        exec(inp);
    }
}

fn exec(inp: Vec<String>) {
    let mut flags: Vec<String> = vec![];
    for arg in &inp {
        if arg.starts_with("-") {
            flags.push(arg[1..].to_owned());
        }
    }
    if inp.len() > 0 {
        match &inp[0][..] {
            "print" => t_print(inp[1..].to_vec()),
            "cd"    => t_cd(inp[1..].to_vec()),
            "file"  => t_file(inp[1..].to_vec()),
            "dir"   => t_dir(inp[1..].to_vec()),
            "var"   => t_var(inp[1..].to_vec()),
            "del"   => t_del(inp[1..].to_vec()),
            "move"  => t_move(inp[1..].to_vec()),
            "if"    => t_if(inp[1..].to_vec()),
            "clear" => t_clear(),
            _       => utils::error("Unknown command")
        }
    }
}

// ==================================================================
// ============================FUNCTIONS=============================
// ==================================================================

fn t_clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn t_print(args: Vec<String>) {
    println!("{}", args.join(" "));
}

fn t_cd(args: Vec<String>){
    if args.len() <= 0 {
        utils::error("Expected at least 1 argument but received none");
        return;
    }

    let path = args.join(" ");

    if !Path::new(&path).exists() {
        utils::error(&format!("Path {} does not exist", &path));
        return;
    }

    let _ = env::set_current_dir(&path);
}

fn t_file(args: Vec<String>) {
    if args.len() != 1 {
        utils::error(&format!("Expected 1 argument but received {}", args.len()));
        return;
    }

    let path = args.join(" ");
    let mut prev_text = "".to_owned();
    if Path::new(&path).is_file() {
        prev_text = fs::read_to_string(&path).unwrap();
    }

    fs::write(&path, utils::readtxt(prev_text)).expect("Unable to write file")
}

fn t_dir(args: Vec<String>) {
    let _ = fs::create_dir(&args[0]);
}

fn t_del(args: Vec<String>) {
    if args.len() != 1 {
        utils::error(&format!("Expected 1 argument but received {}", args.len()));
        return;
    }

    let file = Path::new(&args[0]);
    if !file.exists() {
        utils::error(&format!("Unable to find file or directory: {}", args[0]));
    }

    if file.is_file() {
        let _ = fs::remove_file(&args[0]);
        return;
    }
    let _ = fs::remove_dir(&args[0]);
}

fn t_var(args: Vec<String>) {
    if args.len() != 2 {
        utils::error(&format!("Expected 2 argument but received {}", args.len()));
        return;
    }

    unsafe {
        let name = &args[0];
        for (i, v) in utils::VARS.iter().enumerate() {
            if v.key == name.to_owned() {
                utils::VARS[i] = utils::KeyValuePair {
                    key: v.key.to_owned(),
                    value: args[1].to_owned()
                };
                return;
            }
        }
        utils::VARS.push(utils::KeyValuePair {
            key: name.to_owned(),
            value: args[1].to_owned()
        });
    }
}

fn t_move(args: Vec<String>) {
    if args.len() != 2 {
        utils::error(&format!("Expected 2 argument but received {}", args.len()));
        return;
    }

    if !Path::new(&args[0]).is_file() {
        utils::error(&format!("File {} not found", &args[0]));
        return;
    }

    let _ = fs::rename(&args[0], &args[1]);
}

fn t_if (args: Vec<String>) {
    let inp = args.join(" ");
    if !inp.contains("=>") {
        utils::error("If statement doesn't contain a follow-up.");
        return;
    }
    let mut followup_idx = 1;
    for (i, c) in inp.chars().enumerate() {
        if c == '>' && inp.chars().nth(i-1).unwrap() == '=' {
            followup_idx = i;
            break;
        }
    }
    let mut equ = &inp[..followup_idx-1];
    while equ.ends_with(" ") {
        equ = &equ[..equ.len()-1];
    }
    let mut followup = &inp[followup_idx+1..];
    while followup.starts_with(" ") {
        followup = &followup[1..];
    }
    if utils::expr(equ) == "1" {
        exec(utils::parse(followup.to_owned()));
    }
}
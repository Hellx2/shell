use std::io::prelude::*;
use std::fs::*;
use dirs::*;
use whoami::*;

fn main() {
    let mut aliases: Vec<(String, String)> = vec![];
    println!("Enigma Shell (c) Hellx2 2024");
    let f = format!("{}/.esh", home_dir().unwrap().into_os_string().into_string().unwrap());
    
    if !std::path::Path::new(format!("{}-history", f.as_str()).as_str()).exists() {
        File::create(format!("{}-history", f.as_str()).as_str()).unwrap();
    }
    let mut history = OpenOptions::new().append(true).read(true).open(format!("{}-history", f.as_str()).as_str()).unwrap();
    let j = format!("{}rc", f);
    if !std::path::Path::new(j.as_str()).exists() {
        File::create(&j).unwrap();
    }
    let mut f = File::open(&j).unwrap();
    let mut b: Vec<u8> = vec![];
    f.read_to_end(&mut b).unwrap();
    for i in String::from_utf8(b).unwrap().lines() {
        run(i.to_owned(), &mut aliases, &mut history);
    }

    loop {
        let (/*mut*/ un, /*mut*/ hn, /*mut*/ at, /*mut*/ cdir, /*mut*/ dollar) = ("96", "32", "33", "94", "31");
        // TODO: Add support for themes
        print!("(\x1b[{};1m{}\x1b[0;{}m@\x1b[{cdir}m{} \x1b[{}m{}\x1b[0m) \x1b[{}m$\x1b[0m ", un, username(), hn, hostname(), at, std::env::current_dir().unwrap().file_name().unwrap().to_os_string().into_string().unwrap(), dollar);
        std::io::stdout().flush().unwrap();
        let mut x = String::new();
        std::io::stdin().read_line(&mut x).unwrap();
        run(x, &mut aliases, &mut history);
    }
}

fn exec(command: &str, args: Vec<&str>, aliases: &[(String, String)]) {
    if aliases.iter().any(|a|a.0 == command) {
        let f = aliases.iter().filter(|a|a.0 == command).last().unwrap();
        let j = match f.1.trim().split_once(' ') {
            Some(a) => a,
            None => (f.1.as_str(), "")
        };
        let mut g: Vec<&str> = j.1.split(' ').collect::<Vec<&str>>();
        let mut f = args.to_owned();
        g.append(&mut f);
        let h = aliases.len() - 1 - aliases.iter().rev().position(|a|a.0 == command).unwrap();
        let mut i = aliases[..h].to_vec();
        i.append(&mut aliases[(h+1)..].to_vec());
        exec(j.0, g, &i);
        return;
    }
    let x = read_dir("/bin").unwrap().filter(|a| a.as_ref().unwrap().file_name().into_string().unwrap() == command)
                    .collect::<Vec<Result<DirEntry, std::io::Error>>>();
    if x.is_empty() {
        match command {
            "cd" => std::env::set_current_dir(args[0].trim()).unwrap(),
            "exit" => std::process::exit(0),
            _ => cnf(command),
        }
    } else {
        std::process::Command::new(x[0].as_ref().unwrap().path()).args(args.iter().filter(|a|!a.is_empty())).current_dir(std::env::current_dir().unwrap()).spawn().unwrap().wait().unwrap();
    }
}
// Command Not Found
fn cnf(x: &str) {
    eprintln!("\x1b[31;1mError:\x1b[0m Command not found: {}", x);
}

fn run(x: String, aliases: &mut Vec<(String, String)>, history: &mut File) {
    let y = match x.split_once(' ') {
        Some(a) => (a.0.to_owned(), a.1.to_owned()),
        None => (x.clone(), String::from(""))
    };
    if y.0 == "alias" {
        let b = y.1.split('=').collect::<Vec<&str>>();
        aliases.push((String::from(b[0]), String::from(b[1])));
    } else {
        exec(y.0.trim(), y.1.trim().split(' ').collect::<Vec<&str>>(), aliases);
    }
    std::io::stdout().flush().unwrap();
    history.write_all(x.as_bytes()).unwrap();
}

/*
// TODO: IMPLEMENT THIS
fn parse_file_path(x: String) -> String {
    if x.starts_with('~') {
        vec![home_dir().unwrap().into_os_string().into_string().unwrap(), String::from(&x[1..])].join("")
    } else {
        x
    }
}
*/

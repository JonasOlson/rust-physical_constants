use std::process::Command;
use std::env;

fn main() {
    let out_file = &(env::var("OUT_DIR").unwrap() + "/table.rs");
    Command::new("src/parse_table.py")
        .args(&["src/allascii.txt", out_file])
        .status()
        .unwrap();
}

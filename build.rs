extern crate regex;
use regex::Regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let exponents = Regex::new(r"\^(?P<exponent>-?\d+)").unwrap();

    let f_in = BufReader::new(File::open("src/allascii.txt").unwrap());
    let mut f_out = File::create(env::var("OUT_DIR").unwrap() + "/constants.rs").unwrap();

    for line in f_in.lines()
        .map(|x| x.unwrap())
        .skip_while(|x| !x.contains("-----"))
        .skip(1)
    {
        let mut columns = line.split("  ").map(|x| x.trim()).filter(|x| !x.is_empty());
        let original_name = columns.next().unwrap();
        let name = original_name
            .replace(
                "{220} lattice spacing of silicon",
                "LATTICE_SPACING_220_OF_SILICON",
            )
            .replace("mom.um", "momentum")
            .replace(" ", "_")
            .replace("-", "_")
            .replace(".", "")
            .replace(",", "")
            .replace("(", "")
            .replace(")", "")
            .replace("/", "_PER_")
            .to_uppercase();
        let val = columns.next().unwrap().replace(" ", "").replace("...", "");
        let unit = match columns.skip(1).next() {
            Some(u) => format!(
                "unit: {}",
                exponents
                    .replace_all(u, "<sup>${exponent}</sup>")
                    .replace(" ", "⋅")
            ),
            None => "dimensionless".to_owned(),
        };
        f_out
            .write_fmt(format_args!(
                "/// {} ({})\npub const {}: f64 = {}f64;\n",
                original_name, unit, name, val
            ))
            .unwrap();
    }
}

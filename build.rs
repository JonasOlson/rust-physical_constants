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
    let mut frac_out = File::create(env::var("OUT_DIR").unwrap() + "/constants_pq.rs").unwrap();

    for line in f_in
        .lines()
        .map(|x| x.unwrap())
        .skip_while(|x| !x.contains("-----"))
        .skip(1)
    {
        let mut columns = line.split("  ").map(|x| x.trim()).filter(|x| !x.is_empty());
        let original_name = columns.next().unwrap();
        let name = original_name
            .replace(" ", "_")
            .replace("-", "_")
            .replace(".", "")
            .replace(",", "")
            .replace("(", "")
            .replace(")", "")
            .replace("/", "_PER_")
            .to_uppercase();
        let val = columns.next().unwrap().replace(" ", "").replace("...", "");
        let (unit, unit_org) = match columns.nth(1) {
            Some(u) => (format!(
                "unit: {}",
                exponents
                    .replace_all(u, "<sup>${exponent}</sup>")
                    .replace(" ", "⋅")
                ),
                match u {
                    "(GeV/c^2)^-2" => "c4/GeV2".to_owned(),
                    "u" => "Da".to_owned(),
                    "ohm" => "\u{03A9}".to_owned(),
                    u => u.chars().fold(String::new(), |mut s, c| {
                        if c != '^' {
                            s.push(c)
                        }
                        s
                    }),
                }
            ),
            None => ("dimensionless".to_owned(), "\u{2014}".to_owned())
        };
        f_out
            .write_fmt(format_args!(
                "/// {} ({})\npub const {}: f64 = {}f64;\n",
                original_name, unit, name, val
            ))
            .unwrap();
        frac_out
            .write_fmt(format_args!(
                "/// {} ({})\n\
                pub const {}: unitage::PhysicalQuantity<unitage::Frac, unitage::dim!(\"{}\")> = \n\
                    \tunitage::pq!(unitage::frac!({}), \"{}\", Frac);\n",
                original_name, unit, name, unit_org, val, unit_org
            ))
            .unwrap();
    }
}

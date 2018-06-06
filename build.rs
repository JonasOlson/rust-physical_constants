use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;

fn main() {
    let f_in = BufReader::new(File::open("src/allascii.txt").unwrap());
    let mut f_out = File::create(env::var("OUT_DIR").unwrap() + "/table.rs").unwrap();

    for line in
        f_in.lines()
        .map(|x| x.unwrap())
        .skip_while(|x| !x.contains("-----"))
        .skip(1) {
            let mut words = line.split("  ")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty());
            let original_name = words.next().unwrap();
            let name = original_name
                .replace("{220} lattice spacing of silicon",
                         "LATTICE_SPACING_220_OF_SILICON")
                .replace("mom.um", "momentum")
                .replace(" ", "_")
                .replace("-", "_")
                .replace(".", "")
                .replace(",", "")
                .replace("(", "")
                .replace(")", "")
                .replace("/", "_PER_")
                .to_uppercase();
            let val = words.next().unwrap()
                .replace(" ", "")
                .replace("...", "");
            let unit = match words.skip(1).next() {
                Some(u) => format!("unit: {}", u),
                None => "dimensionless".to_owned()
            };
            f_out.write_fmt(format_args!("/// {} ({})\npub const {}: f64 = {}f64;\n",
                                         original_name, unit, name, val)).unwrap();
        }
}

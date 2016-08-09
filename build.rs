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
                .filter(|x| !(*x).eq(""))
                .take(2);
            let name = words.next().unwrap()
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
            f_out.write_fmt(format_args!("pub const {}: f64 = {}f64;\n", name, val)).unwrap();
        }
}

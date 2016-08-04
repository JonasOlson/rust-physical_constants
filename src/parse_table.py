#!/usr/bin/env python3
# coding: utf-8

from itertools import dropwhile, islice

f_in = open('allascii.txt', 'r')
f_out = open('table.rs', 'w')
f_out.write('pub fn table(name: &str) -> Result<f64, ()> {\nmatch name {\n')
# Loop over lines in the file, dropping everything until we get past the "-------" line and get to the actual data.
for line in islice(dropwhile(lambda l: "----------------------------" not in l,
                             f_in.readlines()),
                   1, None):
    name = line[:60].rstrip()
    value = line[60:85].replace(' ', '').replace('...', '')
    output = '"' + name + '" => Ok(' + value + 'f64),\n'
    f_out.write(output)
f_out.write('_ => Err(())\n}\n}\n')

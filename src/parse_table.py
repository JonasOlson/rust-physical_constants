#!/usr/bin/env python3
# coding: utf-8

import sys
from itertools import dropwhile, islice

f_in = open(sys.argv[1], 'r')
f_out = open(sys.argv[2], 'w')
f_out.write(
    '/// Takes a name of a physical constant and returns its value.\n'
    '///\n'
    '/// Names and units units are found in the [CODATA 2014 table][codata 2014].\n'
    '///\n'
    '/// # Examples\n'
    '///\n'
    '/// ```\n'
    '/// use physical_constants::physical_constants;\n'
    '///\n'
    '/// let epsilon_0 = physical_constants("electric constant").unwrap();\n'
    '/// let mu_0 = physical_constants("mag. constant").unwrap();\n'
    '/// println!("speed of massless particles: {}", 1f64/(epsilon_0*mu_0).sqrt());\n'
    '/// println!("impedance of free space: {}", (mu_0/epsilon_0).sqrt());\n'
    '/// ```\n'
    '///\n'
    '/// [codata 2014]: http://physics.nist.gov/cuu/Constants/Table/allascii.txt\n\n'
    'pub fn physical_constants(name: &str) -> Result<f64, ()> {\n'
    'match name {\n'
)
# Loop over lines in the file, dropping everything until we get past the "-------" line and get to the actual data.
for line in islice(dropwhile(lambda l: "----------------------------" not in l,
                             f_in.readlines()),
                   1, None):
    name = line[:60].rstrip()
    value = line[60:85].replace(' ', '').replace('...', '')
    output = '"' + name + '" => Ok(' + value + 'f64),\n'
    f_out.write(output)
f_out.write('_ => Err(())\n}\n'
            '}\n'
)

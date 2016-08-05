#!/usr/bin/env python3
# coding: utf-8

import sys
from itertools import dropwhile, islice

f_in = open(sys.argv[1], 'r')
f_out = open(sys.argv[2], 'w')

# Loop over the lines in the file, dropping everything until we get past the "-------" line and thus get to the actual data.
for line in islice(dropwhile(lambda l: "----------------------------" not in l,
                             f_in.readlines()),
                   1, None):
    name = (line[:60].rstrip()
            .replace('{220} lattice spacing of silicon',
                     'LATTICE_SPACING_220_OF_SILICON')
            .replace('mom.um', 'momentum')
            .replace(' ', '_')
            .replace('-', '_')
            .replace('.', '')
            .replace(',', '')
            .replace('(', '')
            .replace(')', '')
            .replace('/', '_PER_')
            .upper())
    value = (line[60:85]
             .replace(' ', '')
             .replace('...', ''))
    f_out.write('pub const ' + name + ': f64 = ' + value + 'f64;\n')

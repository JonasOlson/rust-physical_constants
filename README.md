# rust-physical_constants

Rust library providing access to the [CODATA recommended values for physical constants][codata].

## Current functionality

A constant of type `f64` is exposed for each quantity in the [CODATA 2014 table][codata 2014].

For better or for worse, at the moment,

* no quantity names have been changed beyond what is necessary to make them valid Rust identifiers,
* no omissions of possibly redundant quantities have been made, and
* none of the exact values have been recalculated for greater precision (instead, they appear with as many digits as they have in the original listing of quantities).

## Possible future developments

* Use a library (like [`dimensioned`](http://paholg.com/project/dimensioned/)) to return quantities in a form that carries along their physical dimensions and their units.
* Uncertainties
* [Uncertainty correlations](http://physics.nist.gov/cuu/Correlations/)
* Previous CODATA datasets. (Note that comparisons between values from different datasets might be meaningless, as the definitions of the SI units might have changed. The old values can nevertheless be useful for reproducing old calculations.)

[codata]: http://physics.nist.gov/cuu/Constants/
[codata 2014]: http://physics.nist.gov/cuu/Constants/Table/allascii.txt
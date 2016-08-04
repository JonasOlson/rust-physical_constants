# rust-physical_constants

Rust library providing access to the [CODATA recommended values for physical constants](http://physics.nist.gov/cuu/Constants/)

## Current functionality

Reflects the columns "Quantity" and "Value" in the complete [CODATA 2014 table](http://physics.nist.gov/cuu/Constants/Table/allascii.txt). For better or for worse, there are at the moment no changed quantity names, no omissions of possibly redundant quantities, and no recalculated exact values.

## Possible future developments

* Use a library (like [`dimensioned`](http://paholg.com/project/dimensioned/)) to return quantities in a form that carries along their physical dimensions and their units.
* Uncertainties
* [Uncertainty correlations](http://physics.nist.gov/cuu/Correlations/)
* Previous CODATA datasets. (Note that comparisons of values from different datasets might not make sense as the definitions of the SI units might have changed. The old values might nevertheless be useful in reproducing old calculations.)
* Identify a constant using an enum value instead of a string.

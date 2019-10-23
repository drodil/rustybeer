# RustyBeer #

RustyBeer is a CLI tool written in Rust, to calculate values used in the process of brewing beer.

## Installation ##

**TODO**

## Files ##

**TODO**

## Functionality ##

Below is a table of the features currently implemented.

Implemented                   | Function      | Description                                    | Usage
------------------------------|---------------|------------------------------------------------|-------
:white_check_mark:            | ABV           | Calculates ABV from original and final gravity | `abv --og <Original gravity> --fg <Final gravity>`
:negative_squared_cross_mark: | Boil-off      | Calculates the volume needed to be boiled down to for a desired SG | TDB
:negative_squared_cross_mark: | Dilution      | Calculates the SG after dilution | TBD
:white_check_mark:            | Priming       | Beer Priming Calculator                        | `priming --temp <Beer temperature> --amount <Beer volume> --co2_volumes <co2_volumes>`
:white_check_mark:            | SG Correction | Corrects SG reading according to the difference between the measurement temperature and the calibration temperature | `sg_correction --sg <Specific gravity reading> --ct <Calibration temperature> --mt <Measurement temperature>`

This list will expand as ideas and suggestions come in.

## Other Tasks to Do ##

- [ ] Beer recipe file support
- [ ] Unit measurement selection support

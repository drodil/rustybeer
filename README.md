# RustyBeer

RustyBeer is a CLI tool written in Rust, to calculate values used in the process of brewing beer.

## Installation

If you don't already have the toolset installed, you will first need to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).
From the root of the repository, run the following command:
```shell
cargo build
```
You can now run it with:
```shell
./target/debug/rustybeer <subcommand>
```

## Files and Folders

- **src** - The folder containing all source code
  - **calculators** - The folder containing subcommands for performing calculations
    - **abv.rs** - The file containing the subcommand `abv`
    - **mod.rs** - The file defining the names of the structures used within the folder
    - **priming.rs** - The file containing the subcommand `priming`
    - **sg_correction.rs** - The file containing the subcommand `sg_correction`
  - **main.rs** - The file containing the main function
- **Cargo.toml** - The file containing build and dependency infomation
- **LICENSE** - The file containing the terms that this code package is released under
- **README.md** - The file you are currently reading

## Functionality

Below is a table of the features currently implemented.

Implemented                   | Function      | Description                                    | Usage
------------------------------|---------------|------------------------------------------------|-------
:white_check_mark:            | ABV           | Calculates ABV from original and final gravity | `abv --og <Original gravity> --fg <Final gravity>`
:negative_squared_cross_mark: | Boil-off      | Calculates the volume needed to be boiled down to for a desired SG | TDB
:negative_squared_cross_mark: | Dilution      | Calculates the SG after dilution | TBD
:white_check_mark:            | Priming       | Beer Priming Calculator                        | `priming --temp <Beer temperature> --amount <Beer volume> --co2_volumes <co2_volumes>`
:white_check_mark:            | SG Correction | Corrects SG reading according to the difference between the measurement temperature and the calibration temperature | `sg_correction --sg <Specific gravity reading> --ct <Calibration temperature> --mt <Measurement temperature>`

This list will expand as ideas and suggestions come in.

## Other Tasks to Do

- [ ] Beer recipe file support
- [ ] Unit measurement selection support

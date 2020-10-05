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
    - **boil_off.rs** - The file containing the subcommand `boil_off`
    - **mod.rs** - The file defining the names of the structures used within the folder
    - **priming.rs** - The file containing the subcommand `priming`
    - **sg_correction.rs** - The file containing the subcommand `sg_correction`
  - **main.rs** - The file containing the main function
- **Cargo.toml** - The file containing build and dependency infomation
- **LICENSE** - The file containing the terms that this code package is released under
- **README.md** - The file you are currently reading
- **CONTRIBUTING.md** - Contribution guidelines for this repository

## Functionality

Below is a table of the features currently implemented.

Implemented                   | Function      | Description                                    | Usage
------------------------------|---------------|------------------------------------------------|-------
:white_check_mark:            | ABV           | Calculates ABV from original and final gravity | `abv --og <Original gravity> --fg <Final gravity>`
:hourglass_flowing_sand:      | Boil-off Gravity| Calculates the volume needed to be boiled down to for a desired SG | `boil_off --current_gravity <current_gravity> --wort_volume <wort_volume> <--target_volume <target_volume>|--desired_gravity <desired_gravity>>`
:white_check_mark:            | Dilution      | Calculates the SG after dilution               | `diluting --sg <Current specific gravity> --cv <Current volume> --tv <Target volume>`
:white_check_mark:            | Priming       | Beer Priming Calculator                        | `priming --temp <Beer temperature> --amount <Beer volume> --co2_volumes <co2_volumes>`
:white_check_mark:            | SG Correction | Corrects SG reading according to the difference between the measurement temperature and the calibration temperature | `sg_correction --sg <Specific gravity reading> --ct <Calibration temperature> --mt <Measurement temperature>`
:white_check_mark:            | Beer style    | Finds beer styles matching given parameters    | `beer_style (--og <Original gravity>) (--fg <Final gravity>) (--abv <Alcohol by volume>) (--ibu <International bittering units> (--color <SRM color>)`

This list will expand as ideas and suggestions come in.

## Other Tasks to Do

- [ ] Beer recipe file support
- [ ] Unit measurement selection support

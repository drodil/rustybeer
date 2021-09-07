![](https://repository-images.githubusercontent.com/215803204/ca492f0d-e920-4dd9-9375-afa03c70fc96)
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-9-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://github.com/drodil/rustybeer/blob/master/LICENSE)
[![Contributors](https://img.shields.io/github/contributors/drodil/rustybeer.svg?style=flat)]()
[![Issues](https://img.shields.io/github/issues-raw/drodil/rustybeer.svg?maxAge=25000)](https://github.com/drodil/rustybeer/issues)
[![PRs](https://img.shields.io/github/issues-pr/drodil/rustybeer.svg?style=flat)](https://github.com/drodil/rustybeer/pulls)

RustyBeer is a CLI tool / web server written in Rust, to calculate values used in the process of brewing beer.

Live server running at [https://rustybeer.herokuapp.com/](https://rustybeer.herokuapp.com/)

## Installation

If you don't already have the toolset installed, you will first need to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).
From the root of the repository, run the following command:

```shell
cargo build
```

### Running CLI

You can now run the CLI tool with:

```shell
cargo run --bin rustybeer <subcommand>
```

### Running HTTP server

To start the server:

```shell
cargo run --bin rustybeer-server
```

You can access the OpenAPI UI from http://localhost:3000/docs. To change the
port number, you can define environment variable PORT.

## Testing

Tests can be ran by calling:

```shell
cargo test
```

If you would like to run only one test, you can do this by specifying the test name:

```shell
cargo test -- --nocapture <test name>
```

## Files and Folders

- [**rustybeer**](rustybeer) - The folder containing everything for the business logic
  - [**src**](rustybeer/src) - The folder containing the business logic source code
    - [**calculators**](rustybeer/src/calculators) - The folder containing calculators to be used in lib or CLI tool
  - [**Cargo.toml**](rustybeer/Cargo.toml) - The file containing build and dependency infomation
- [**rustybeer-cli**](rustybeer-cli) - The folder containing everything for the CLI
  - [**src**](rustybeer-cli/src) - The folder containing the CLI source code
    - [**commands**](rustybeer-cli/src/commands) - The folder containing subcommands for CLI
    - [**main.rs**](rustybeer-cli/src/main.rs) - The file containing the main function
  - [**Cargo.toml**](rustybeer-cli/Cargo.toml)- The file containing build and dependency infomation
- [**rustybeer-server**](rustybeer-server) - The folder containing the HTTP server implementation
  - [**src**](rustybeer-server/src) - The folder containing server source code
    - [**handlers**](rustybeer-server/src/handlers) - HTTP request handlers
  - [**Cargo.toml**](rustybeer-server/Cargo.toml) - The file containing server build and dependency information
- [**Cargo.toml**](Cargo.toml) - The file containing build and dependency infomation
- [**CONTRIBUTING.md**](CONTRIBUTING.md) - Contribution guidelines for this repository
- [**LICENSE**](LICENSE) - The file containing the terms that this code package is released under
- [**README.md**](README.md) - The file you are currently reading

## Acronyms

Beer brewing has a lot of acronyms that have a meaning. This table is to help
out with figuring out what everything means:

Acronum      | Description
-------------|---------------------------------
ABV          | Alcohol By Volume
ABW          | Alcohol By Weight
OG           | Original Gravity
FG           | Final Gravity
SG           | Specific Gravity
IBU          | International Bittering Units

## CLI Functionality

Below is a table of the features currently implemented.

Implemented              | Function                                                           | Description                                                        | Usage
-------------------------|--------------------------------------------------------------------|--------------------------------------------------------------------|-------
:white_check_mark:       | [ABV](rustybeer-cli/src/commands/abv.rs)                           | Calculates ABV from OG and FG or FG from OG and ABV                | `abv --og <Original gravity> (--fg <Final gravity>) (--abv <Alcohol by volume>)`
:white_check_mark:       | [ABV <-> ABW](rustybeer-cli/src/commands/alcohol_volume_weight.rs) | Calculates alcohol by weight (ABW) from  alcohol by volume (ABV)   | `abv_abw --percent <alcohol percentage> (--total_volume <total beer volume>) (--total_density <density of beer in g/cm³) (--reverse)`
:white_check_mark:       | [Beer style](rustybeer-cli/src/commands/beer_style.rs)             | Finds beer styles matching given parameters                        | `beer_style (--og <Original gravity>) (--fg <Final gravity>) (--abv <Alcohol by volume>) (--ibu <International bittering units> (--color <SRM color>)`
:hourglass_flowing_sand: | [Boil-off Gravity](rustybeer-cli/src/commands/boil_off.rs)         | Calculates the volume needed to be boiled down to for a desired SG | `boil_off --current_gravity <current_gravity> --wort_volume <wort_volume> <--target_volume <target_volume>|--desired_gravity <desired_gravity>>`
:white_check_mark:       | [Calories](rustybeer-cli/src/commands/calories.rs)                 | Calculates calories by volume from OG and FG or from ABV           | `calories (--og <Original gravity>) (--fg <Final gravity>) (--abv <Alcohol by volume>) (--volume <Beer volume>)`
:white_check_mark:       | [Dilution](rustybeer-cli/src/commands/diluting.rs)                 | Calculates the SG after dilution                                   | `diluting --sg <Current specific gravity> --cv <Current volume> --tv <Target volume>`
:white_check_mark:       | [FG](rustybeer-cli/src/commands/fg.rs)                             | Calculates FG from OG and yeast attenuation                        | `fg --og <Original gravity> --att <Yeast attenuation>`
:white_check_mark:       | [Num Of Bottles](rustybeer-cli/src/commands/num_bottles.rs)        | Calculates the number of bottles required for a given volume       | `num_of_bottles --volume <volume>`
:white_check_mark:       | [Priming](rustybeer-cli/src/commands/priming.rs)                   | Beer Priming Calculator                                            | `priming --temp <Beer temperature> --amount <Beer volume> --co2_volumes <co2_volumes>`
:white_check_mark:       | [SG Correction](rustybeer-cli/src/commands/sg_correction.rs)       | Corrects SG reading for differences between measurement and calibration temperatures | `sg_correction --sg <Specific gravity reading> --ct <Calibration temperature> --mt <Measurement temperature>`
:white_check_mark:       | [Yeast Viability](rustybeer-cli/src/commands/yeast_viability.rs)   | Estimates yeast viability based off production date | `yeast-viability --pd <Production date> --cc <Cell count> --f <Date format>`

This list will expand as ideas and suggestions come in.

## Other Tasks to Do

See [Issues](https://github.com/drodil/rustybeer/issues)

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://drodil.kapsi.fi"><img src="https://avatars0.githubusercontent.com/u/1178319?v=4" width="100px;" alt=""/><br /><sub><b>drodil</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=drodil" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/mlatief"><img src="https://avatars3.githubusercontent.com/u/462098?v=4" width="100px;" alt=""/><br /><sub><b>mlatief</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=mlatief" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/ProgrammerJoe93"><img src="https://avatars3.githubusercontent.com/u/56159225?v=4" width="100px;" alt=""/><br /><sub><b>Joseph Russell</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=ProgrammerJoe93" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/flauntingspade4"><img src="https://avatars1.githubusercontent.com/u/48335751?v=4" width="100px;" alt=""/><br /><sub><b>flauntingspade4</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=flauntingspade4" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/i-jey"><img src="https://avatars1.githubusercontent.com/u/25993326?v=4" width="100px;" alt=""/><br /><sub><b>Ilakkiyan Jeyakumar</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=i-jey" title="Code">💻</a></td>
    <td align="center"><a href="http://linkedin.com/in/tommilligan477"><img src="https://avatars2.githubusercontent.com/u/12255914?v=4" width="100px;" alt=""/><br /><sub><b>Tom Milligan</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=tommilligan" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/rogercyyu"><img src="https://avatars0.githubusercontent.com/u/45835736?v=4" width="100px;" alt=""/><br /><sub><b>Roger Y</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=rogercyyu" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/Sampas"><img src="https://avatars1.githubusercontent.com/u/1084004?v=4" width="100px;" alt=""/><br /><sub><b>Sampsa Sarjanoja</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=Sampas" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/1jz"><img src="https://avatars0.githubusercontent.com/u/1187260?v=4" width="100px;" alt=""/><br /><sub><b>Philip Golovin</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=1jz" title="Code">💻</a></td>
  </tr>
</table>

<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
=======

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

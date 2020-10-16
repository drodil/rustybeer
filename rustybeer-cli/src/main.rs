use clap::{App, AppSettings};

mod commands;

fn main() {
    let app = App::new("RustyBeer")
        .version("0.1")
        .subcommand(commands::abv::add_subcommand())
        .subcommand(commands::beer_style::add_subcommand())
        .subcommand(commands::boil_off::add_subcommand())
        .subcommand(commands::diluting::add_subcommand())
        .subcommand(commands::priming::add_subcommand())
        .subcommand(commands::sg_correction::add_subcommand())
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    match app.subcommand_name() {
        Some(s) => match s {
            "abv" => {
                commands::abv::do_matches(&app);
            }
            "beer_style" => {
                commands::beer_style::do_matches(&app);
            }
            "boil_off" => {
                commands::boil_off::do_matches(&app);
            }
            "diluting" => {
                commands::diluting::do_matches(&app);
            }
            "priming" => {
                commands::priming::do_matches(&app);
            }
            "sg_correction" => {
                commands::sg_correction::do_matches(&app);
            }
            _ => println!("Not recognised subcommand"),
        },
        None => println!("A subcommand must be provided"),
    }
}

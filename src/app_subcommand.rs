// Trait that all subcommands must implement
pub trait AppSubCommand {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>;
    fn do_matches<'c>(&self, matches: &ArgMatches<'c>);
}


use rustybeer::yeast::yeast::read_yeasts;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "yeast")]
/// Search yeast information
pub struct YeastOptions {
    #[structopt(short, long)]
    /// Search by yeast name
    name: String,
}

pub fn search_and_print(_opt: YeastOptions) {
    let yeast_file = Path::new("rustybeer/src/yeast/res/yeast.txt");
    let yeasts = read_yeasts(yeast_file);
    match yeasts {
        Ok(yeasts) => {
            // TODO filter yeast by options
            for y in yeasts.iter() {
                println!("{:?}", y);
            }
        }
        Err(error) => println!("Problem with yeasts: {:?}", error),
    }
}

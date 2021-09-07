use rustybeer::yeasts::YEASTS;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "yeast")]
/// Search yeast information
pub struct YeastOptions {
    #[structopt(short, long)]
    /// Search by yeast name
    name: Option<String>,
}

pub fn search_and_print(opt: YeastOptions) {
    if let Some(name) = opt.name {
        let criteria = name.to_lowercase();
        for yeast in YEASTS.iter() {
            if yeast.name.to_lowercase().contains(&criteria) {
                println!("{:?}", yeast);
            }
        }
    } else {
        for yeast in YEASTS.iter() {
            println!("{:?}", yeast);
        }
    }
}

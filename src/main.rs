use rastrgraph::{self, Cli};
#[allow(unused_imports)]
use raster::{self, Color, Image};
use structopt::StructOpt;

#[allow(unused_variables)]
fn main() {
    let args = Cli::from_args();
    let path = rastrgraph::get_image_path(args.image).unwrap(); // Need to handle this error better than using unwrap, will use an `if let` statment with `ok_or_err`


    let image = raster::open(&path);

    println!("{}", path);
    std::process::exit(0);

}

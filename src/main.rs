use rastrgraph::{self, Cli};
use structopt::StructOpt;

#[allow(unused_variables)]
fn main() {
    let args = Cli::from_args();
    if let Err(e) = rastrgraph::run_args(args) {
	eprintln!("an error has occured: {:?}",e);
	std::process::exit(1);
    }
    std::process::exit(0);
    
}

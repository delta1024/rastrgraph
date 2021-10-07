use structopt::StructOpt;
use std::path::PathBuf;
#[allow(unused_imports)]
use raster::{self, Color, Image};

type RastrResult<T> = Result<T, RastrError>;

#[derive(Debug)]
pub struct RastrError(String); 	

#[derive(StructOpt, Debug)]
#[structopt(about = "get image info from the command line")]
pub struct Cli {
    /// Generate Histogram
    #[structopt(short = "-g" , long)]
    pub histogram: bool,
    /// The file to look for
    #[structopt(parse(from_os_str))]
    pub image: std::path::PathBuf,
}


/// runs logic on what functions to call.
pub fn run_args(args: Cli) -> RastrResult<()> {
    let image_path = get_image_path(args.image)?;
    if args.histogram {
	generate_histogram(image_path)?;
    }
    Ok(())
}

fn generate_histogram(_path: String) -> RastrResult<()> {
    todo!{};
    // Ok(())
}
/// Gets the absolute file path for the given input
pub fn get_image_path(path: PathBuf) -> RastrResult<String> {
    let parent = path.as_path()
	.canonicalize()
	.map_err(|err|
		 RastrError(format!("Error reading `{}`: {}",
				    match path.to_str() {
					Some(p) => p,
					None => "hello",
				    },
				    err)))?;

    let parent = match parent.to_str() {
	Some(p) => { p },
	None => { panic!("unknown error") },
    };

    Ok(String::from(parent))
}

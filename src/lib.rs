use structopt::StructOpt;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};

/// holds all CLI arguments
#[derive(StructOpt, Debug)]
pub struct Cli {
    /// The file to look for
    #[structopt(parse(from_os_str))]
    pub image: std::path::PathBuf,
}

/// Gets the absolute file path for the given input
pub fn get_image_path(path: PathBuf) -> std::io::Result<String> {
    let parent = match path.as_path().parent() {
	Some(a) => a,
	None => {
	    return Err(Error::new(ErrorKind::NotFound, "directory not found"));
	},
    };

    let parent = std::fs::canonicalize(parent)?;

    let file_name = match path.file_name() {
	Some(a) => a,
	None => {
	    return Err(Error::new(ErrorKind::NotFound, "no file found"));
	}
    };

    let file_path = format!("{}/{}", parent.to_str()
			    .ok_or(Error::new(
				ErrorKind::Other, "couldn't get string from parent"))?,
			    file_name.to_str()
			    .ok_or(Error::new(
				ErrorKind::Other, "couldn't get string from file name"))?);

    Ok(file_path)
}

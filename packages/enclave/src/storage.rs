use log::error;
use std::io::Write;
use std::sgxfs::SgxFile;
use crate::error::Error;


pub fn seal(data: &[u8], filepath: &str) -> Result<(), Error> {
    let mut file = SgxFile::create(filepath).map_err(|_err| {
        error!("error creating file {}: {:?}", filepath, _err);
        Error::GenericError
    })?;

    file.write_all(data).map_err(|_err| {
        error!("error writing to path {}: {:?}", filepath, _err);
        Error::GenericError
    })
}

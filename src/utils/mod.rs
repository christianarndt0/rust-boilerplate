//! Basic utilities module
//! 
//! PUT SMALL MISC STUFF IN HERE THAT DOESN'T NECESSARILY FIT ANYWHERE ELSE


pub mod logging;


/// Determine the current working directory and append folder by folder (by file) as per the given input vector.
pub fn path_from_cwd(dirs: Vec<String>) -> Result<std::path::PathBuf, std::io::Error> {
    let mut path = std::env::current_dir()?;

    for d in &dirs {
        path = path.join(d);
    }

    Ok(path)
}

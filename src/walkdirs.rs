use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};


#[derive()]
pub struct WalkResults {
    pub items: Vec<walkdir::DirEntry>,
    pub errs: Vec<walkdir::Error>
}

pub fn walkdirs(p: PathBuf, d: u16) -> WalkResults {

    let mut results = WalkResults {
        items: Vec::<DirEntry>::new(),
        errs: Vec::<Error>::new(),
    };

    gatherdirs(&mut results, p, d);

    return results;
}


fn gatherdirs(results: &mut WalkResults, p: PathBuf, depth: u16) {
    if depth > 0 {

        for entry in WalkDir::new(p) {
            match entry {
                Ok(de) => {
                    results.items.push(de);
                },
                Err(e) => {
                    results.errs.push(e);
                }
            }
        }

    }
}


#[cfg(test)]
mod tests {
    use super::walkdirs;


    #[test]
    fn can_gather_current_dir() {
        match std::path::Path::new(".").canonicalize() {
            Ok(pb) => {
                walkdirs(pb, 1);  // 1 => only return info from curr dir; don't recurse                
            },

            Err(e) => {
                assert_eq!(true, false, "Failed to cannonicalize curr dir: {:?}", e);
            }
        }
        
    }

    #[test]
    fn can_recurse_current_dir() {
        match std::path::Path::new(".").canonicalize() {
            Ok(pb) => {
                walkdirs(pb, u16::MAX);                
            },

            Err(e) => {
                assert_eq!(true, false, "Failed to cannonicalize curr dir: {:?}", e);
            }
        }
        
    }

}
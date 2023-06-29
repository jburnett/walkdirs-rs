use std::io::{Error};
use std::path::PathBuf;
use std::{fs};


#[derive()]
pub struct WalkResults {
    pub items: Vec<PathBuf>,
    pub errs: Vec<Error>
}

pub fn walk(p: PathBuf, d: u16) -> WalkResults {

    let mut results = WalkResults {
        items: Vec::<PathBuf>::new(),
        errs: Vec::<Error>::new(),
    };

    gatherdirs(&mut results, p, d);

    return results;

}

fn gatherdirs(results: &mut WalkResults, p: PathBuf, depth: u16) {

    println!("gatherdirs at depth {}", depth);

    if depth > 0 {

        // Aquire the dir iterator
        match fs::read_dir(p) {
            Ok(itr) => {
                // Walk the iterator
                for item in itr {
                    // add the item (or error) the results
                    match item {
                        Ok(de) => {
                            results.items.push(de.path());
                            if de.path().is_dir() {
                                // recurse into the dir
                                gatherdirs(results, de.path().to_path_buf(), depth - 1);
                            }
                        },
                        Err(e) => {
                            results.errs.push(e);
                        }
                    }
                }
            },

            // Error acquiring the iterator; add it to the error results
            Err(e) => {
                    results.errs.push(e);
            }
        }

    }

}

#[cfg(test)]
mod tests {
    use super::walk;


    #[test]
    fn can_gather_current_dir() {
        match std::path::Path::new(".").canonicalize() {
            Ok(pb) => {
                walk(pb, 1);  // 1 => only return info from curr dir; don't recurse                
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
                walk(pb, u16::MAX);                
            },

            Err(e) => {
                assert_eq!(true, false, "Failed to cannonicalize curr dir: {:?}", e);
            }
        }
        
    }

}
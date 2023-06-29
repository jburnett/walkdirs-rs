use std::io::{Error};
use std::path::PathBuf;
use std::{fs};


#[derive()]
struct WalkResults {
    items: Vec<PathBuf>,
    errs: Vec<Error>
}

pub fn walk(p: PathBuf, d: u16) {

    let mut results = WalkResults {
        items: Vec::<PathBuf>::new(),
        errs: Vec::<Error>::new(),
    };

    gatherdirs(&mut results, p, d);

    let count = results.items.len();

    for item in results.items {
        println!("{:?}", item);
    }

    println!("\n===== walk found {:?} items", count);

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
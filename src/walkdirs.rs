use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};


#[derive()]
pub struct WalkResults {
    pub items: Vec<walkdir::DirEntry>,
    pub errs: Vec<walkdir::Error>
}

pub fn walkdirs(p: PathBuf, d: usize) -> WalkResults {

    let mut results = WalkResults {
        items: Vec::<DirEntry>::new(),
        errs: Vec::<Error>::new(),
    };

    gatherdirs(&mut results, p, d);

    return results;
}


fn gatherdirs(results: &mut WalkResults, p: PathBuf, depth: usize) {
    if depth > 0 {

        let walker = WalkDir::new(p).into_iter();
        for entry in walker.filter_entry(|e| {
            !is_git_dir(e) && e.depth() <= depth
        } ) {
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


fn is_git_dir(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.eq(".git"))
        .unwrap_or(false)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}


#[cfg(test)]
mod tests {
    use std::usize;
    use rstest::*;

    use super::walkdirs;


    #[rstest]
    #[case("./testdata", 0, 0)] // TODO: should levels=0 be disallowed?
    #[case("./testdata", 1, 3)] // target dir only
    #[case("./testdata", 2, 6)] // target dir and direct children
    // full depth beginning at target dir
    #[case("./testdata", usize::MAX, 8)]
    #[case("./testdata/a/aa", usize::MAX, 2)]
    #[case("./testdata/emptydir", usize::MAX, 1)]
    // relative pathing should be same as absolute
    #[case("./testdata/a/", usize::MAX, 6)]
    #[case("./testdata/a/aa/..", usize::MAX, 6)]
    fn test_walkdirs(#[case] path: &'static str, #[case] levels: usize, #[case] expected: usize ) {

        match std::path::Path::new(path).canonicalize() {
            Ok(pb) => {
                let res = walkdirs(pb, levels);
                // assert_ne!(res.items.len(), 0);
                assert_eq!(res.items.len(), expected, "Found {:?} items", res.items.len());
            },

            Err(e) => {
                assert_eq!(true, false, "Failed to cannonicalize curr dir: {:?}", e);
            }
        }
    }

}
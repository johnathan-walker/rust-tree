use walkdir::WalkDir;

fn main() {
    let dirString =".";
    find_root(dirString)
}

pub fn find_root(directory_string : String) {
    for entry in WalkDir::new(directory_string).follow_links(true) {
        let entry= entry.unwrap();
        println!("{}", entry.path().display());
    }
}
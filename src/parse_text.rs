use std::fs::File;
use std::path::Path;

//returns a file
pub fn open(path:&str) -> File {
    let path = Path::new(path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    return file
}

//turns a text file into an iterator of lines
pub fn file_to_iter(file:File) -> std::io::BufReader<std::fs::File> {
    let reader = std::io::BufReader::new(file);
    return reader
}
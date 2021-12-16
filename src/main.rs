/* Challenge:
(4)


    Write a function that given a directory, recursively finds all files with a given file
    extension in that directory and all sub-directories, and counts the number of lines
    in the file and prints it to stdout.
*/

///
/// # Arguments
///
/// * `dir` - A string that holds the path of the directory
/// * `ext` - A string that holds the file extension
///
/// # Examples
///
/// Run:
///         /target/debug/file_crawler [FILEPATH] [FILE EXTENSION]
///     or   
///
///         cargo run [FILEPATH] [FILE EXTENSION]
///
/// Output:
///     ["target/debug/file_crawler", "tmp", "txt"]
///     FILENAME: "tmp/test.txt", LINE COUNT: 5
///     FILENAME: "tmp/tmp_level_2/test_level_2.txt", LINE COUNT: 5
///     Done. Ok(())
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::io::{Error, ErrorKind};
use std::path::Path;

fn count_lines<P>(filename: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().count())
}

fn match_extension(ext: &String, path_ext: Option<&OsStr>) -> io::Result<bool> {
    match path_ext {
        None => Ok(false),
        Some(os_str) => match os_str.to_str() {
            Some(_ext) => {
                if ext == _ext {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Err(Error::new(ErrorKind::Other, "Possibly an invalid Unicode.")),
        },
    }
}

fn crawl(dir: &String, ext: &String) -> io::Result<()> {
    let meta = fs::metadata(dir).unwrap();

    if meta.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                match crawl(&path.into_os_string().into_string().unwrap(), ext) {
                    Ok(()) => (),
                    Err(_) => continue,
                }
            } else if match_extension(ext, path.extension()).unwrap() == true {
                println!(
                    "FILENAME: {:?}, LINE COUNT: {:?}",
                    path,
                    count_lines(&path)?
                );
            }
        }
    } else {
        panic!("{} Is NOT a directory", dir);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() == 3 {
        println!("Done. {:?}", crawl(&args[1], &args[2]));
    } else {
        println!("Insufficient arguments.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::{Write};

    #[test]
    fn test_count_lines() {
        let dir = match tempdir() {
            Err(why) => panic!("couldn't create directory: {}", why),
            Ok(dir) => dir,
        };
        let file_path = dir.path().join("my-temporary-note.txt");
        let display = file_path.display();
        let mut file = match File::create(&file_path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        write!(file, "Hello World! \n 1 \n 2").unwrap();
        assert_eq!(count_lines(&file_path).unwrap(), 3);
    }

    #[test]
    fn test_match_extension() {
        let file_path = Path::new("my-temporary-note.txt");
        assert_eq!(match_extension(&"txt".to_string(), file_path.extension()).unwrap(), true);
        assert_eq!(match_extension(&"tx".to_string(), file_path.extension()).unwrap(), false);
    }

    #[test]
    #[should_panic(expected = "Is NOT a directory")]
    fn test_crawl() {
        let dir = match tempdir() {
            Err(why) => panic!("couldn't create directory: {}", why),
            Ok(dir) => dir,
        };
        match fs::create_dir(dir.path().join("sub_dir")) {
            Err(why) => panic!("couldn't create directory: {}", why),
            Ok(dir) => dir,
        };
        let file_path = dir.path().join("my-temporary-note.txt");
        let sub_dir_file_path = dir.path().join("sub_dir/my-temporary-note.txt");
        let display = file_path.display();
        let mut file = match File::create(&file_path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        let mut sub_dir_file = match File::create(&sub_dir_file_path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        write!(file, "Hello World! \n 1 \n 2").unwrap();
        write!(sub_dir_file, "Hello World! \n 1 \n 2").unwrap();
        assert_eq!(crawl(&dir.path().to_str().unwrap().to_string(), &"txt".to_string()).is_ok(), true);

        //should panic, not a directory
        assert_eq!(crawl(&file_path.into_os_string().into_string().unwrap(), &"txt".to_string()).is_ok(), false);
    }
}

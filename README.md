# file_crawler
    Given a directory, recursively finds all files with a given file
    extension in that directory and all sub-directories, and counts the number of lines
    in the file and prints it to stdout.

# Arguments
 * `dir` - A string that holds the path of the directory
 * `ext` - A string that holds the file extension

# Examples

Run:
         /target/debug/file_crawler [FILEPATH] [FILE EXTENSION]
     or
         cargo run [FILEPATH] [FILE EXTENSION]

Output:
     ["target/debug/file_crawler", "tmp", "txt"]
     FILENAME: "tmp/test.txt", LINE COUNT: 5
     FILENAME: "tmp/tmp_level_2/test_level_2.txt", LINE COUNT: 5
     Done. Ok(())

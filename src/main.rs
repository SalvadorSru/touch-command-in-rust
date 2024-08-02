use std::{
    env,
    fs::{create_dir_all, File},
    path::Path,
    process,
};

fn main() {
    // Collect command-line arguments
    let args: Vec<_> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        process::exit(1); // Exit with an error code
    }

    // Get the file path from the arguments
    let requested_path: &str = &args[1];

    // Create a Path object from the path
    let path = Path::new(requested_path);

    // Check if the path is a file or directory based on its extension or trailing slashes
    if path.extension().is_some()
        || path.file_name().is_some() && path.file_name().unwrap().to_str().unwrap().contains('.')
    {
        // Path is assumed to be a file
        // Extract the parent directory from the path
        let folders = path.parent().unwrap_or_else(|| Path::new("."));

        // Attempt to create necessary directories
        if let Err(e) = create_dir_all(folders) {
            eprintln!("Error creating directories: {}", e);
            process::exit(1); // Exit with an error code
        }

        // Attempt to create the file; if it already exists, it will be opened
        if let Err(e) = File::create(path) {
            eprintln!("Error creating file: {}", e);
            process::exit(1); // Exit with an error code
        }
    } else {
        // Path is assumed to be a directory
        // Attempt to create the directory (and any necessary parent directories)
        if let Err(e) = create_dir_all(path) {
            eprintln!("Error creating directories: {}", e);
            process::exit(1); // Exit with an error code
        }
    }
}

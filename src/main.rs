use std::{
    env,
    fs::{create_dir_all, File},
    path::Path,
    process,
};

fn main() {
    // Collect command-line arguments
    let args: Vec<_> = env::args().collect();

    // Print arguments for debugging purposes (can be removed in production)
    println!("Arguments: {:?}", args);

    // Check if the correct number of arguments is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        process::exit(1); // Exit with an error code
    }

    // Get the file path from the arguments
    let requested_file: &str = &args[1];

    // Create a Path object from the file path
    let path = Path::new(requested_file);

    // Extract the parent directory from the path
    let folders = path.parent().unwrap_or_else(|| Path::new(""));

    // Attempt to create necessary directories
    if let Err(e) = create_dir_all(folders) {
        eprintln!("Error creating directories: {}", e);
        process::exit(1); // Exit with an error code
    }

    // Attempt to create the file; if it already exists, it will be opened
    if let Err(e) = File::create(requested_file) {
        eprintln!("Error creating file: {}", e);
        process::exit(1); // Exit with an error code
    }
}

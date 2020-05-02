use std::fs;
use subprocess;
use tempfile::NamedTempFile;


/// Create a String in VIM(https://www.vim.org/) if it is installed via a temporary file.  Designed
/// for CLI use.
///
/// ```
/// use vim_edit::{vim_create};
///
/// fn something() {
///   let vim_string: String = vim_create();
///   println!("You wrote: {}", vim_string)
/// }
/// ```
pub fn vim_create() -> String {
    let some_file = NamedTempFile::new();
    let file_path = String::from(some_file.unwrap().path().to_string_lossy());

    subprocess::Exec::cmd("vim")
        .arg(&file_path)
        .join()
        .unwrap();

    fs::read_to_string(&file_path).unwrap()
}

/// Edit a String from a Rust app in VIM(https://www.vim.org/) if it is installed via a temporary
/// file.  Designed for CLI use.
///
/// ```
/// use vim_edit::{vim_edit};
///
/// fn something() {
///   let mut vim_string: String = String::from("This is an example.");
///   vim_string = vim_edit(vim_string);
///   println!("You edit the string to be: {}", vim_string)
/// }
/// ```
pub fn vim_edit(current_string: String) -> String {
    let some_file = NamedTempFile::new();
    let file_path = String::from(some_file.unwrap().path().to_string_lossy());
    fs::write(&file_path, current_string)
        .expect("Something went wrong with writing the temp file");

    subprocess::Exec::cmd("vim")
        .arg(&file_path)
        .join()
        .unwrap();

    fs::read_to_string(&file_path).unwrap()
}

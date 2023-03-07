use std::env;
use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("disallow_file_extensions")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Sondre Lillebø Gundersen <sondrelg@live.no>")
        .about("Disallow file extensions")
        .arg(
            Arg::new("extensions")
                .short('e')
                .long("extensions")
                .value_name("EXTENSIONS")
                .help("Comma separated list of file extensions")
        )
        .arg(
            Arg::new("filenames")
                .help("Changed files")
                .required(true)
                .action(ArgAction::Append)
                .help("Filenames to check")
        )

        .get_matches();

    let banned_extensions = matches
        .get_many::<String>("extensions")
        .unwrap_or_default()
        .collect::<Vec<_>>();

    let exit_code: bool = matches
        .get_many::<String>("filenames")
        .unwrap_or_default()
        .any(|filename| {

            let extension = ".".to_string() + &std::path::Path::new(filename)
                .extension()
                .map(|ext| ext.to_string_lossy().to_string())
                .unwrap_or_default();

            if banned_extensions.contains(&&extension) {
                println!("{} uses a disallowed file extension", filename);
                true
            } else {
                false
            }
        });

    std::process::exit(if exit_code { 1 } else { 0 });
}

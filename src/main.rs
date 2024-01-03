mod organizer;
use clap::{App, Arg};

fn main() {
    let matches = App::new("File Organizer")
        .version("1.0")
        .author("Your Name")
        .about("Organizes files into directories based on file type")
        .arg(
            Arg::with_name("directory")
                .short('d')
                .long("directory")
                .value_name("DIRECTORY")
                .help("Sets the target directory to organize")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let directory = matches.value_of("directory").unwrap();
    println!("Organizing files in directory: {}", directory);

    organizer::organize_files(directory);
}


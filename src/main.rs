use clap::{Arg, ArgAction, Command};
use insertion_sort_rs::*;

fn main() {
    let cmd = Command::new("insertion-sort")
        .about("Insertion sort on the command line.")
        .arg_required_else_help(true)
        .author("João Vitor Cunha Assumpção | JVCAv1")
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .conflicts_with("file")
                .help("Manually typed values.")
                .long_help(
                    "Manually typed values. separated by spaces,\n".to_owned()
                        + "e.g:   -l 9 3 2 7 4 3 21 2 491 2 1",
                )
                .action(ArgAction::Set)
                .num_args(1..),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file-path")
                .conflicts_with("list")
                .help("Path to the file in which the contents will be sorted")
                .long_help(
                    "Path to the file in which the contents will be sorted,\n".to_owned()
                        + "the values must be separated by spaces and be on the\n"
                        + "last line of the file.\n"
                        + "e.g:    -f /path/to/file.txt",
                )
                .action(ArgAction::Set)
                .num_args(1),
        )
        .get_matches();

    //for ARRAY
    if cmd.get_one::<String>("list").is_some() {
        let sorted_nums = arg_list(&cmd);
        print!("Sorted: ");
        sorted_nums.iter().for_each(|x| print!("{}", x));
        println!(" ");
    }

    if cmd.get_one::<String>("file").is_some() {
        let sorted_nums = arg_file(&cmd);
        print!("Sorted: ");
        sorted_nums.iter().for_each(|x| print!(" {}", x));
        println!(" ");
    }
}

use clap::{builder::ValueParser, Arg, ArgAction, Command};
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
                .help("Manually typed values separated by spaces, e.g 9 3 2 7 4 ")
                .action(ArgAction::Set)
                .num_args(1..),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file-path")
                .conflicts_with("list")
                .help("Path to the file in which the contents will be sorted")
                .action(ArgAction::Set)
                .num_args(1)
                .value_parser(ValueParser::path_buf()),
        )
        .get_matches();

    //for ARRAY
    if let Some(_) = cmd.get_one::<String>("list") {
        let sorted_nums = arg_list(&cmd);
        print!("Sorted: ");
        for i in 0..sorted_nums.len() {
            print!("{} ", sorted_nums[i]);
        }
        println!(" ");
    }

    if let Some(_) = cmd.get_one::<String>("file") {
        todo!()
    }
}

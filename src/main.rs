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
                .help("Manually typed values separated by spaces, e.g 9 3 2 7 4 ")
                .action(ArgAction::Set)
                .num_args(1..),
        )
        .get_matches();

    //for ARRAY
    if let Some(_) = cmd.get_one::<String>("list") {
		arg_list(&cmd);
	}
}

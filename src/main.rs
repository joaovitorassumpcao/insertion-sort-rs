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
                .help("Manually typed values separated by spaces, e.g 9 3 2 7 4 ")
                .action(ArgAction::Set)
                .num_args(1..),
        )
        .get_matches();

    //for ARRAY
    if let Some(list) = cmd.get_one::<String>("list") {
        let mut nums: Vec<i32> = cmd
            .get_many::<String>("list")
            .expect("contains_id")
            .map(|x| x.parse::<i32>().expect("error parsing to i32"))
            .collect();

        let sorted_nums = insertion_sort(&mut nums);
        print!("Sorted: ");
        for i in 0..sorted_nums.len() {
            print!("{} ", sorted_nums[i]);
        }
        println!(" ");
    }
}

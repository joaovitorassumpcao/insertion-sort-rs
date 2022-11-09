use insertion_sort_rs::*;
use clap::{Arg, Command, ArgAction};

fn main() {
    let matches = Command::new("insertion-sort")
        .about("Insertion sort on the command line.")
        .arg_required_else_help(true)
        .author("João Vitor Cunha Assumpção | JVCAv1")
        .arg(
            Arg::new("array")
                .short('l')
                .long("list")
                .help("Manually typed values separated by spaces, e.g '1 3 2 77 4 [...]'")
                .action(ArgAction::Set)
                .num_args(1..)
            )
        .get_matches();

    match matches. {
        Some(("typed", typed_matches)) => {
            //for ARRAY
            if typed_matches.contains_id("array") {
                let mut nums: Vec<i32> = typed_matches
                    .get_many::<String>("array")
                    .expect("contains_id")
                    .map(|x| x.parse::<i32>().expect("error parsing to i32"))
                    .collect();

                let sorted_nums = insertion_sort(&mut nums);
                print!("Sorted: ");
                for i in 0..sorted_nums.len(){
                    print!("{} ", sorted_nums[i]);
                }
                println!(" ");
            }
        },
        Some((&_,_)) => panic!("Invaid argument!"),
        None => panic!("No argument!"),
    }

}

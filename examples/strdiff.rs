use ronp::diff;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        println!("few arguments");
        std::process::exit(1);
    }

    let a = &args[1];
    let b = &args[2];

    println!("A:{}", a);
    println!("B:{}", b);

    let diff = diff::Diff::new(a.chars().collect::<Vec<_>>(), b.chars().collect::<Vec<_>>());
    let res = diff.build();
    println!("editdistance: {}", res.ed());
}

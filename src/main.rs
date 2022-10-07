fn print_help() -> ! {
    println!(
        "Usage:
    random-element <...elements>

    or

    cat ./lines.txt | random-element
"
    );
    std::process::exit(1);
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.contains(&String::from("--help")) {
        print_help();
    }

    let elements = if args.len() > 0 {
        args
    } else {
        // stdin mode activated
        use std::io::*;
        BufReader::new(stdin())
            .lines()
            .collect::<Result<_>>()
            .expect("read lines from stdin")
    };

    let index = rand::random::<f64>() * elements.len() as f64;

    let element = &elements[index.floor() as usize];

    print!("{element}");
}

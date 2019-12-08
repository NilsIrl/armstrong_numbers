use std::io::BufRead;

struct Digits {
    n: usize,
    count: usize,
}

impl Digits {
    fn new(n: usize) -> Self {
        Digits { n, count: 0 }
    }
}

impl Iterator for Digits {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.n > 0 {
            let d = Some(self.n % 10);
            self.n /= 10;
            d
        } else {
            None
        }
    }
}

fn main() {
    let matches = clap::App::new("Armstrong Numbers")
        .version("0.1")
        .author("Nils André <nils@nilsand.re>")
        .about("Checks if the inputted numbers are amrstrong numbers or not.")
        .arg(
            clap::Arg::with_name("numbers")
                .help("The number to check.")
                .multiple(true),
        )
        .arg(
            clap::Arg::with_name("file")
                .help("The file containing the numbers to test. '-' indicated standard input.")
                .short("f")
                .long("file")
                .default_value("-"),
        )
        .get_matches();
    let buf_reader;
    let mut numbers_arg;
    let mut iterator;
    let mut lines;
    for number in match matches.values_of("numbers") {
        Some(numbers) => {
            numbers_arg = numbers.map(|number| number.parse().unwrap());
            &mut numbers_arg as &mut dyn Iterator<Item = usize>
        }
        None => match matches.value_of("file").unwrap() {
            "-" => {
                iterator = std::iter::from_fn(|| {
                    if atty::is(atty::Stream::Stdin) {
                        print!("Please enter a number to be tested: ");
                        use std::io::Write;
                        std::io::stdout().flush().unwrap();
                    }
                    loop {
                        match std::io::stdin().lock().lines().next() {
                            Some(line) => match line.unwrap().trim().parse() {
                                Ok(number) => return Some(number),
                                Err(_) => (),
                            },
                            None => {
                                if atty::is(atty::Stream::Stdin) {
                                    println!("\nGoodbye 👋");
                                }
                                return None;
                            }
                        }
                    }
                });
                &mut iterator as &mut dyn Iterator<Item = usize>
            }
            file => {
                buf_reader = std::io::BufReader::new(std::fs::File::open(file).unwrap());
                lines = buf_reader
                    .lines()
                    .map(|line| line.unwrap().parse().unwrap());
                &mut lines as &mut dyn Iterator<Item = usize>
            }
        },
    } {
        if Digits::new(number)
            .map(|digit| digit.pow((number as f32).log10().ceil() as u32))
            .sum::<usize>()
            == number
        {
            println!("{} is an Armstrong number.", number);
        } else {
            println!("{} isn't an Armstrong number.", number);
        }
    }
}

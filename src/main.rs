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
    use std::io::BufRead;
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
    let stdin;
    let mut lock;
    let mut reader;
    let mut buf_reader;
    let mut numbers_arg;
    for number in match matches.values_of("numbers") {
        Some(numbers) => {
            numbers_arg = numbers.map(|number| number.parse().unwrap());
            &mut numbers_arg as &mut dyn Iterator<Item = usize>
        }
        None => {
            buf_reader = match matches.value_of("file").unwrap() {
                "-" => {
                    stdin = std::io::stdin();
                    lock = stdin.lock();
                    &mut lock as &mut dyn BufRead
                }
                file => {
                    reader = std::io::BufReader::new(std::fs::File::open(file).unwrap());
                    &mut reader
                }
            }
            .lines()
            .map(|line| line.unwrap().parse().unwrap());
            &mut buf_reader as &mut dyn Iterator<Item = usize>
        }
    } {
        if Digits::new(number)
            .map(|digit| digit.pow((number as f32).log10().ceil() as u32))
            .sum::<usize>()
            == number
        {
            println!("{} is an Armstrong numbers.", number);
        } else {
            println!("{} isn't Armstrong numbers.", number);
        }
    }
}

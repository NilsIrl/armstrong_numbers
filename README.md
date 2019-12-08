# armstrong_numbers

High performance ~~multithreaded~~ armstrong numbers

# Usage

```bash
$ cargo run -- --help
```

This program accepts input in 3 different ways:

1. Interactive

```bash
$ cargo run
Please enter a number to be tested: **9**
9 is an Armstrong number.
Please enter a number to be tested: **10**
10 isn't Armstrong number.
Please enter a number to be tested: **153**
153 is an Armstrong number.
Please enter a number to be tested: **154**
154 isn't an Armstrong number.
Please enter a number to be tested: **<C-d>**
Goodbye 👋
```

`CTRL+D` cuts standard input causing the program to stop, like you would with
other interactive UNIX programs.

2. Standard input

```bash
$ cargo run <<< "$RANDOM"
29090 isn't an Armstrong number.
$ cargo run <<< "$RANDOM"
4173 isn't an Armstrong number.
$ cargo run <<< "$RANDOM"
7377 isn't an Armstrong number.
$ cargo run <<< "$RANDOM"
8346 isn't an Armstrong number.
$ cargo run <<< "$RANDOM"
11985 isn't an Armstrong number.
$ cargo run <<< "$RANDOM"
16056 isn't an Armstrong number.
$ cargo run <<< "$RANDOM"
19356 isn't an Armstrong number.
$ cargo run <<< "$RANDOM"
2463 isn't an Armstrong number.
$ cargo run <<< "$RANDOM"
10556 isn't an Armstrong number.
$ cargo run <<< "$RANDOM"
26870 isn't an Armstrong number.
$ cargo run <<< "$RANDOM"
17655 isn't an Armstrong number.
```

3. Arguments

```bash
$ cargo run -- 9 10 153 154
9 is an Armstrong number.
10 isn't an Armstrong number.
153 is an Armstrong number.
154 isn't an Armstrong number.
$ cargo run -- $(seq 0 1000)
...
```

4. Files

```bash
$ seq 0 100000 > file.txt
$ cargo run -- -f file.txt | grep -v "isn't"
0 is an Armstrong numbers.
1 is an Armstrong numbers.
2 is an Armstrong numbers.
3 is an Armstrong numbers.
4 is an Armstrong numbers.
5 is an Armstrong numbers.
6 is an Armstrong numbers.
7 is an Armstrong numbers.
8 is an Armstrong numbers.
9 is an Armstrong numbers.
153 is an Armstrong numbers.
370 is an Armstrong numbers.
371 is an Armstrong numbers.
407 is an Armstrong numbers.
1634 is an Armstrong numbers.
8208 is an Armstrong numbers.
9474 is an Armstrong numbers.
54748 is an Armstrong numbers.
92727 is an Armstrong numbers.
93084 is an Armstrong numbers.
```


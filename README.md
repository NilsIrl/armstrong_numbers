# armstrong_numbers

High performance ~~multithreaded~~ armstrong numbers

# Usage

```bash
$ cargo run -- --help
```

This program accepts input in 3 different ways:

1. Standard input

```bash
$ cargo run
9
9 is an Armstrong numbers.
10
10 isn't Armstrong numbers.
153
153 is an Armstrong numbers.
154
154 isn't Armstrong numbers.
```

2. Arguments

```bash
$ cargo run -- 11 $(seq 0 1000)
...
```

3. Files

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


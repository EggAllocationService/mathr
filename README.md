# mathr

A command-line time based expression evaluator for libmapper.

## Installation

```bash
# clone the repository
git clone https://github.com/EggAllocationService/mathr.git
cd mathr

# build the project
cargo build --release

# install to PATH
cargo install --path .
```

## Examples

```bash
mathr "sin(t)" # a signal with a sine wave with a period of 3.4 milliseconds
mathr "sin(t/(pi * 1000))" # a signal with a sine wave with a period of 1 second

mathr "sin(t/(pi * 500)) * cos(t/(pi*500))" # Cool wavy thing
```

You can write any expression using common math functions (basically any operation in the rust standard libary), and the variable `t` which represents the time since the program started in milliseconds.

You can also use the options `-n` and `-d` to change the signal and device names, respectively. The `-r` option changes the time between evaluations, default is 10 milliseconds.
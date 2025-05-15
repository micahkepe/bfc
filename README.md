# `bfc` Brainf%ck x86-64 Compiler

> [!IMPORTANT]
> Currently targets x86-64 macOS (via `clang`)

From [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck):

> Brainfuck is an esoteric programming language created in 1993 by Swiss student
> Urban Müller. Designed to be extremely minimalistic, the language consists
> of only eight simple commands, a data pointer, and an instruction pointer.

## ⚙️ Installation

```bash
# Clone and build
git clone https://github.com/your‑org/bfc.git
cd bfc
cargo install --path .        # installs `bfc` into your $CARGO_HOME/bin
```

## 💻 Usage

```
Usage: bfc [OPTIONS] <INPUT>

Arguments:
  <INPUT>  The path string to the Brainf%ck source file

Options:
  -o, --output <OUTPUT>  The path string to the output executable [default: a.out]
  -v, --verbose...       Increase logging verbosity
  -q, --quiet...         Decrease logging verbosity
  -e, --execute          Whether to assemble and link the generated '.asm' file
  -h, --help             Print help
  -V, --version          Print version
```

Without the `--execute` flag, a `.asm` file in the same directory will be
generated.

With `--execute`, `bfc` will run:

1. `nasm -f macho64 yourprog.asm -o yourprog.o`
2. `clang -arch x86_64 -e _main yourprog.o -o a.out`

> [!NOTE]
> The `tests/` directory contains some sample `*.bf` programs that you can run.

## "Hello World!"

Save `hello_world.bf`:

```bf
++++++++                Set Cell #0 to 8
[
    >++++               add 4 to Cell #1
    [                   inner loop
        >++>+++>+++>+   fill cells 2–5
        <<<<-           decrement Cell #1
    ]
    >+>+>- >>+ [<] <-   mix and clear
]
>>.>---.+++++++..+++.  “Hello”
>>.>---.>+++.------.--------.  “ World!”
>>+.>++.                 newline
```

Compile and run:

```bash
bfc hello.bf --execute
./a.out
# Hello World!
```

## License

This repository is licensed under the MIT License. See [LICENSE](./LICENSE) for
more details.

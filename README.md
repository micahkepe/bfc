# `bfc` Brainf%ck x86-64 Compiler

> [!IMPORTANT]
> Currently targets x86-64 macOS (via `clang`)

From [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck):

> Brainfuck is an esoteric programming language created in 1993 by Swiss student
> Urban Müller. Designed to be extremely minimalistic, the language consists
> of only eight simple commands, a data pointer, and an instruction pointer.

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

## License

This repository is licensed under the MIT License. See [LICENSE](./LICENSE) for
more details.

# Daytona

Daytona is an [EVMC] compatible Ethereum VM kit. It aims to support a variety of VM implementations
written in Rust, such as Parity's, SputnikVM, CitaVM.

Currently it only supports Parity's EVM through [standalone-parity-evm].

## Build

Install Rust first, then:

```shell
$ cargo build
```

should result in an EMVC compatible shared library.

## Maintainer(s)

- Alex Beregszaszi

## License

Apache-2.0

[EVMC]: https://github.com/ethereum/evmc
[standalone-parity-evm]: https://github.com/axic/standalone-parity-evm

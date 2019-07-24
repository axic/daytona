# Daytona

Daytona is an [EVMC] compatible Ethereum VM kit.

It aims to support a variety of VM implementations written in Rust, such as [Parity]'s, [SputnikVM] and [CitaVM].
The main goal is to benchmark and compare these implementations.

*P.S. let me know if there are other Rust EVM implementations out there.*

Currently it only supports Parity's EVM through [standalone-parity-evm].

## Build

Install Rust first, then:

```shell
$ cargo build --release
```

should result in an EMVC compatible shared library.

## Maintainer(s)

- Alex Beregszaszi

## License

Apache-2.0

[EVMC]: https://github.com/ethereum/evmc
[Parity]: https://github.com/paritytech/parity-ethereum
[SputnikVM]: https://github.com/ethereumproject/evm-rs
[CitaVM]: https://github.com/cryptape/cita-vm
[standalone-parity-evm]: https://github.com/axic/standalone-parity-evm

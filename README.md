# Daytona

Daytona is an [EVMC] compatible Ethereum VM kit.

It aims to support a variety of VM implementations written in Rust, such as [Parity]'s, [CitaVM],
[SputnikVM], [SputnikVM (fork)], [evm-rs] (discontinued), [evm-rs (fork)] and [rust-evm].

The main goal is to benchmark, compare and fuzz these implementations.

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

**Note:** While this repository is Apache-2.0 licensed, the resulting binary linked with Parity's EVM will be licensed under GPL-3.0.

[EVMC]: https://github.com/ethereum/evmc
[Parity]: https://github.com/paritytech/parity-ethereum
[SputnikVM]: https://github.com/ETCDEVTeam/sputnikvm
[SputnikVM (fork)]: https://github.com/etclabscore/sputnikvm
[evm-rs]: https://github.com/ethereumproject/evm-rs
[evm-rs (fork)]: https://github.com/etclabscore/evm-rs
[rust-evm]: https://github.com/sorpaas/rust-evm
[CitaVM]: https://github.com/cryptape/cita-vm
[standalone-parity-evm]: https://github.com/axic/standalone-parity-evm

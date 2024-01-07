# Würth Radio Driver

This crate imeplements a driver for the Würth Elektronik Radio module. Being tested with Metis-II Wirelss M-BUS.

## Usage

Add an entry to your `Cargo.toml`:

```toml
[dependencies]
wurth-radio-rs = "0.1.0"
```

You can also enable the optional `defmt` feature.

## Running the examples

You can run one of the examples like this, remembering to substitute your own serial device instead of `/dev/ttyUSB0`.

```shell
cargo run --example test -- /dev/ttyUSB0
```

To see what's going on under the hood, you can turn on trace logging by setting the environment variable `RUST_LOG=trace`.

## References

- [Metis-II Reference Manual Version 3.7](https://www.we-online.com/components/products/manual/2607021183000_Metis-II%20260702118300x%20Manual_rev3.7.pdf)

## License

Licensed under [The MIT License](https://opensource.org/license/mit/).

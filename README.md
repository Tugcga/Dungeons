## What is it

This is port of the [Promethean](https://github.com/valantonini/Promethean) library for the 2d-dungeon generation into Python, AssemblyScript and Rust. README of the Python version is [here](https://github.com/Tugcga/Dungeons/tree/main/python), AssemblyScript version is [here](https://github.com/Tugcga/Dungeons/tree/main/assemblyscript/promethean), Rust version is [here](https://github.com/Tugcga/Dungeons/tree/main/rust/promethean).

## Benchmarks

The next table contains time of the generation process for different map sizes. For AssemblyScript version we use NodeJS to execute WASM module. Rust version we measure native complied application and also WASM module (also executed in NodeJS).

Map size | Python | AssemblyScript (WASM, 29 kb) | Rust (native) | Rust (WASM, 51 kb)
--- | --- | --- | -- | --
32 x 32 | 0.09 sec | 0.011 sec | 0.00027 sec | 0.00079 sec
64 x 64 | 0.50 sec | 0.089 sec | 0.0019 sec | 0.0041
128 x 128 | 1.40 sec | 0.24 sec | 0.0076 | 0.017
192 x 192 | 2.31 sec | 0.46 sec | 0.013 | 0.031
256 x 256 | 3.55 sec | 0.68 sec | 0.021 | 0.048

Compiled into WASM AssemblyScript version is nearly x5.5 times faster than Python, Rust WASM version is nearly x14 times faster than AssemblyScript version, Rust native version is nearly x2 times faster than WASM version.

## What is it

This is port of the [Promethean](https://github.com/valantonini/Promethean) library for the 2d-dungeon generation into Python and AssemblyScript. README of the Python version is [here](https://github.com/Tugcga/Dungeons/tree/main/python), of the AssemblyScript version is [here](https://github.com/Tugcga/Dungeons/tree/main/assemblyscript/promethean).

## Benchmarks

The next table contains time of the generation process for different map sizes. For AssembluScript version we use Node.js to execute WASM module.

Map size | Python | AssemblyScript 
--- | --- | ---
32 x 32 | 0.09 sec | 0.011 sec
64 x 64 | 0.66 sec | 0.089 sec
128 x 128 | 1.51 sec | 0.24 sec
192 x 192 | 2.56 sec | 0.46 sec
256 x 256 | 3.91 sec | 0.68 sec

Compiled into WASM AssemblyScript version is nearly x6 times faster than Python.

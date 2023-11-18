# semaphore-witness-example

This is an example template for building a circuit to be used with the rust native witness generator circom-witness-rs. 
Besides building the required graph file, it also shows an example to use 

## Usage
Pass the absolute path to your circuits in the `WITNESS_CPP` env var.
In the example below, everything is in the project directory.

```
WITNESS_CPP="$(pwd)"/src/semaphore.circom cargo build --release
```

This will produce a `graph.bin` file in the root project folder, which contains the execution graph of the witness generator. 
You will need to pass this file during runtime of the libary later.

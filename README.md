## Summary
This code implements a Rust program that searches for files bigger than 100mb in a given directory or the whole file system if no filepath is given. It uses the `clap` and `std` libraries for command line argument parsing and file size calculation, respectively.

## Example Usage
```rust
// Run the program with a specific directory path
$ ./sizer -p /path/to/directory

// Run the program without specifying a directory path (searches the whole file system)
$ ./sizer

// Run the program with a specific directory path and custom file size threshold
$ ./sizer -p /path/to/directory -s 200
```

## Code Analysis
### Inputs
- `matches`: A `clap::ArgMatches` object that contains the parsed command line arguments.
___
### Flow
1. Initialize the logger for logging purposes.
2. Parse the command line arguments using `clap` to get the `matches` object.
3. Get the value of the `path` argument from the `matches` object.
4. If a `path` value is provided, convert it to a `PathBuf` object and assign it to `home_dir`.
5. Get the value of the `size` argument from the `matches` object.
6. If a `size` value is provided, assign it to `size` as an `Option<&String>`.
7. If a `path` value is provided, call the `entry` function from the `sizer` library with `home_dir` and `size` as arguments.
8. If no `path` value is provided, create a `PathBuf` object for the root directory ("/") and assign it to `root_dir`.
9. Call the `entry` function from the `sizer` library with `root_dir` and `size` as arguments.
___
### Outputs
None. The function performs file size calculations and logging based on the provided command line arguments.
___

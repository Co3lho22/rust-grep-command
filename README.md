# rust-grep-command
This is a Rust command-line tool inspired by the project "AN I/O PROJECT: BUILDING A COMMAND LINE PROGRAM" from the book "The Rust Programming Language". The program mimics the functionality of the grep command by searching for a specified string within a file and displaying the matching lines.

## Features
- Search for a specific string in a file.
- Case-sensitive and case-insensitive search options.
- Error handling for incorrect inputs and file access issues.
## Getting Started
### Prerequisites
- Ensure you have Rust installed on your system.
### Installation
1. Clone the repository:

```sh
git clone https://github.com/yourusername/rust-grep-command.git
cd rust-grep-command
```

2. Build the project:

```sh
cargo build --release
```

### Usage
Run the program with the following command:

```sh
cargo run <QUERY> <FILE_PATH>
```

Replace `<QUERY>` with the string you want to search for and `<FILE_PATH>` with the path to the file you want to search in.

Example:

``` sh
cargo run to poem.txt
```

### Case Insensitivity
To perform a case-insensitive search, set the `CASE_INSENSITIVE` environment variable:

```sh
CASE_INSENSITIVE=1 cargo run <QUERY> <FILE_PATH>
```

## Project Structure
- `src/main.rs`: Entry point of the program. Handles command-line arguments and orchestrates the execution.
- `src/lib.rs`: Contains the core functionality of the grep command including configuration parsing, file reading, and search logic.
- `tests`: Unit tests for the search functionality.

## Running Tests
To run the tests, use the following command:

```sh
cargo test
```

## Example
Given the file `poem.txt` with the following content:

```txt
Copy code
The road not taken.
Two roads diverged in a yellow wood,
And sorry I could not travel both
And be one traveler, long I stood
And looked down one as far as I could
To where it bent in the undergrowth;
```

Running the command:

```sh
cargo run road poem.txt
```

Output:

```sh
[
    "The road not taken.",
    "Two roads diverged in a yellow wood,"
]
```


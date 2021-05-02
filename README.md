A simple simulation of terminal tool "grep" written in Rust.

Based on [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) book.

Program is divided in 3 parts:

- **main.rs** handles calling the functions defined in **lib.rs** and also handles printing errors to screen if anything fails:
  - Takes command line input and puts it in `let args`
  - Constructs `Config` based on `args` (If an error occurs, program prints the error message to `stderr` and exits)
  - Calls function `run` from **lib.rs** (If an error occurs, program prints the error message to `stderr` and exits)

<br>

- **lib.rs** defines struct `Config` and its creation, implements searching functions and also contains a test module
  - `Config` stores data about what to search, where and how:
    - `query: String` contains the word we are searching for
    - `filename: String` specifies path to the file in which we're searching
    - `is_case_sensitive: bool` specifies if the search is case-sensitive (if "you" is different from "YOu")
  - `fn new(args: &[String]) -> Result<Config, &str>` creates a `Config` object based on given `args`
    - Takes `query` and `filename` from `args` and sets `is_case_sensitive` based on an environment variable
    - Returns `Result` that either holds the `Config` object or a message about failure
  - `fn search<'a>(word: &str, content: &'a str) -> Vec<&'a str>` Searches (case-sensitively) for every line in `content` with sequence `word`
  - `fn search_case_insensitive<'a>(word: &str, content: &'a str) -> Vec<&'a str>` is same as `search`, except it matches case-insensitively by [lower-casing](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase) everything before checking
  - `fn run(config: Config) -> Result<(), Box<dyn Error>>`
    - Runs the searching method specified by `config` and prints the output to screen
    - Returns a `Result` that holds nothing in case of success and an error in case of failure (more about `Box<dyn Error>` [here](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html))

<br>

- Testing module contains `test1`for testing case-sensitive search and `test2` for case-insensitive search
  - `#[cfg(test)] mod tests` means the module will only be included in test builds (we specify it since we have no need for it in production build)
  - Both `test1` and `test2` have `#[test]` attributes that tell **Rust** to run them as tests
  - Both `test1` and `test2` specify their own `query` and `content` and use `assert_eq!` [macro](https://doc.rust-lang.org/book/ch19-06-macros.html) to verify that their respective search function gives expected result

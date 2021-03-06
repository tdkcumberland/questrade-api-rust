# Questrade API w/ Rust multirepo

Implementation of the Questrade API wrapper as well as my [Portfolio Rebalancer](https://github.com/tdkcumberland/questrade_portfolio_rebalancer) in a Rust multirepo style project.

I used Rust a bit at work to build tools and I want to improve as well as build tools that I can reuse for future personal projects. This very much is a work-in-progress. Currently experimenting with [Polars](https://www.pola.rs/) which is supposedly the fastest dataframe framework now for both Rust **and** Python

### Prerequisites

Requirements for the software and other tools to build, test and push 
- [Rust](https://www.rust-lang.org/)
- [Git](https://www.git-scm.com/book/en/v2/Appendix-A%3A-Git-in-Other-Environments-Git-in-Bash)

## Project structure

This project is set up as Rust [multirepo/workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)

One would have the **bins** where the actual running/executing code as well as compilation targets; and **libs** where different modules can be used as utlities in any other bins/libs.

### Sample Tests

With how Rust set up a lib project, I find it very easy to do test driven development (TDD). All my polars codes so far starts from test. Below is command line to run the tests present in the polars library.

    cargo test --package polars --lib -- tests --nocapture

To run individual test with an exact name

    cargo test --package polars --lib -- tests::test_raw_json_to_polars_df --exact --nocapture

You can find for information on how to control your testing in Rust [here](https://doc.rust-lang.org/book/ch11-02-running-tests.html)

## Authors

  - **Timothy Cumberland**


## License

This project is licensed under the [CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/)

## Acknowledgments

  - README template by **Billie Thompson** - [Billie's Github Profile](https://github.com/PurpleBooth)
  - Inspiration: learning rust as well as a newer dataframe framework alternative to Pandas

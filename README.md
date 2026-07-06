Print compact tests output in terminal as sentences.

## Installation

```sh
cargo install cargo-rdoc
```

## Usage

In any Rust project with tests, run:

```sh
cargo rdoc
```

## Example

`tests::resets_number_to_zero` is printed as `tests - resets number to zero`

`tests::submod::exec_fn_sends_request_to_api` is printed as `tests::submod - exec sends request to api`

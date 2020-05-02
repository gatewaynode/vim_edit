VIM Edit
==============
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/gatewaynode/vim_edit/Rust)

A simple set of functions to create and edit individual Rust Strings with [VIM](https://www.vim.org/).  
May support more options and features in the near future.

Example
-------

*Cargo.toml*
```
[dependencies]
vim_edit = "0.1.0"
```

*main.rs*
```
use vim_edit::{vim_create, vim_edit}

fn main() {
    let mut our_input: String = vim_create();
    println!("You created in vim: {}", our_input);
    our_input = vim_edit(our_input);
    println!("Final edited value is: {}", our_input)
}

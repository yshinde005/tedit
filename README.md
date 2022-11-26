# tedit
A terminal based text editor made from scratch in rust programming language.

## Description 
Tedit is created in Rust, which is a fairly new yet one of the most popular open source programming languages due to it's vast communtiy of contributes. Tedit has all the basic features that makes a text editor optimum for a programmer's usage . 
CRUD operations are implemented within the application.

## To-do
- [ ] Syntax highlighting
- [ ] Search function
- [ ] auto-complete syntax recommandations

## Installation
For tedit you need to have rust and cargo installed in your system.
```ssh 
rustc --version && cargo --version
```
   if you see a version number then you have them. If not, then install [rust](https://doc.rust-lang.org/book/ch01-01-installation.html) and [cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html).

clone the repo with
```ssh
git clone https://github.com/yshinde005/tedit
```
Navigate to tedit
```ssh
cd tedit/
```
To execute and create a new file in tedit
```ssh
cargo run
```
to work in pre-existing file
```ssh
cargo run file-address
```
To exit tedit ``` Ctrl-q ``` and to save changes press ```Ctrl-s```

[![Build Status](https://travis-ci.com/lewis-weinberger/rinfo.svg?branch=master)](https://travis-ci.com/lewis-weinberger/rinfo)

Simple helper utility to print useful information about [RAMSES](https://bitbucket.org/rteyssie/ramses/src/master/) outputs. Needs an installation of Rust, but otherwise no external dependencies. After cloning, simply run `cargo build --release` in the parent directory to compile the binary. Should then be used in the directory containing RAMSES outputs (i.e `output_XXXXX` directories). Note: I don't know if the `info_XXXXX.txt` varies across versions of RAMSES, so this may need tweaking if your version has a different formatting.

Mostly this was a project to play around with Rust! It was inspired by the project described in [Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) of the Book.

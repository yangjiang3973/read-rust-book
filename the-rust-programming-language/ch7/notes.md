# Packages and Crates

A crate can come in two forms: a library or a binary crate.

Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library”.

A package is a bundle of one or more crates that provides a set of functionality.

A package contains a Cargo.toml file that describes how to build those crates.

## Path

two kinds of paths: absolute and relative.

An absolute path starts from a crate root.

A relative path starts from the current module and uses self, super, or an identifier in the current module.

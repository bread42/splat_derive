## About

This is a crate providing a Splat derive macro, which generates a splat method for the struct deriving it.

## The `splat` method

The `splat` method is commonly defined for structs with numeric fields of the same type. It takes a value `v` and returns an instance of the struct where each field is set to `v`.

This crate provides a macro that generates a `splat` method for any struct that has fields which are all of the same type. However, the type shared by each field must implement Clone.

## Usage

Add the following to your Cargo.toml

```toml
[dependencies]
splat_derive = "0.1.0"
```

# multipart-legacy-client

The library [multipart](https://github.com/abonander/multipart) doesn't work with hyper 0.11 as of September 2017. This crate is a temporary workaround: a thin wrapper around (parts of) the client portion of `multipart` that uses hyper 0.10.

This allows multipart to be used by crates that also use hyper 0.11.

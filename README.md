# Private Loggers (and other crates) in Rust FFI Libraries

When building libraries in Rust for consumption via FFI in other languages,
there's a question of what to do about global variables in other crates.
In particular, the Rust ecosystem mostly uses the `log` crate for logging.
Library crates are supposed to use the `log` crate's global LOGGER object
to send logs; applications are supposed to register a logger
exactly once, at the beginning of main. This is important because only one
logger can ever be registered.

What about FFI libraries? The application consuming your code probably isn't in
the Rust ecosystem, so it can't register its own logger. Your library could
register a logger, and provide an interface for the application to register
interest in messages from that logger. But that runs afoul of the requirement
that only the application is supposed to register a logger. And what happens if
some application links your library _and_ some other Rust FFI library that
registers a logger? Whichever library registers its logger first wins.

What we really want is for each FFI library to have its own copy of the `log`
crate, with its own version of the global `log::LOGGER` object.

It turns out Rust has already solved this problem once, for dependency
resolution. [Thanks to @sagebind for an excellent blog post on the topic,
which helped me figure this out](https://stephencoakley.com/2019/04/24/how-rust-solved-dependency-hell).
Rust can use two versions of the same crate in one library, thanks to "name
mangling" and a "disambiguator." It assigns different names to symbols from
each of the crates.

Cargo controls the disambiguator by passing `-C metadata=...` flag to rustc, which
can be specified multiple times. Cargo does dependency resolution to figure out
what it needs to provide for the metadata flag in order to ensure that if
multiple versions of a crate need to be linked, they don't conflict with each
other.

You can use the RUSTFLAGS environment variable to add your own `-C metadata=...`
flags, which Cargo passes along to its rustc invocations. The normal metadata
flags still get passed and respected, so this doesn't break the normal
functioning of the disambiguator.

    RUSTFLAGS="-C metadata=something_unique" cargo build -v

When you build your library with an invocation like the above, all of your
library's dependencies have their names mangled uniquely. So even if someone
links your library, and some other Rust FFI library that has the same
dependencies, (a) the symbols won't conflict, and (b) you can interact with
globals in your dependencies without affecting the other library.

This has a bit of a downside: If an application links in a lot of Rust FFI
libraries that all use this approach, it will wind up with a bunch of
independent copies of common Rust code. Whether this tradeoff is worthwhile
depends on your intended use for the library. I think in most libraries intended
for general use, making dependencies "private" via this technique is the right
solution.

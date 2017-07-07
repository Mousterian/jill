
build.rs sounds like a good idea but it's super-cpu intensive if you are using intellij (constant rebuilding in the
background).

In the interests of getting anything done, I'm falling back to running bindgen from the command line to create FFI stubs.

Experience shows that bindings without documentation of how they were generated are pretty hard to maintain or alter.

Using rust-bindgen 0.26.2 from the command line.

If you don't have bindgen installed already:

~~~
cargo install bindgen --vers "0.26.2"
# and go and make a cup of tea, it'll take a while
~~~

once it's built and installed, from the directory containing this file:

~~~
bindgen --no-unstable-rust -o src/bindings.rs -l jack ./jack.h
~~~

the only difference between the raw bindings generated and the lib.rs is the addition of pragmas to allow non-rusty code
~~~
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
~~~
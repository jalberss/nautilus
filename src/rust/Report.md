# ** Rust in Nautilus **

John Albers and Peter Dinda

JohnAlbers@protonmail.com
pdinda@northwestern.edu

## Why Rust and an Overview of Rust in Nautilus 
Rust is a systems programming language from Mozilla that focuses on
safety, performance, and productivity. For the nonbelievers in strong,
statically languages, Rust can be the way into the strongly typed
cult. The language has a lot of elements of ML (as in Metalanguage,
not gradient descent), such as pattern matching, sum types that are
nice to use, and easy to use Higher Order Functions, so your Racket
friends `filter`, `map`, and `fold` make the cut. Furthermore, you get
all the niceties of newer languages, like templates, asynchronous IO,
and a decent build system and package manager. In short, the language
itself does not allow two mutable references to objects at a time. The
compiler will explicitly disallow this, or you can also move the
checking to runtime. Naturally, this leads to some pain points. The
real kicker is that this really opens up some incredible alias
analysis optimizations. Unfortunately, the Rust compiler does not take
advantage of this yet (At least, that is the understanding that I have
come to have. Simone may now be upset if he ever were to read this
document). The language is obviously still in its infancy and can have
some growing pains associated with that. Furthermore, the language is
on a 6 week release cycle, which can mean that code may work one day
and give different behavior the next, especially since we are dealing
with some of the lower level implementation details of the
language. This guide is written with `rustc 1.36 nightly` in mind. The
current stable version is `rustc 1.37`. There have been errors that
magially resolve themselves, and errors that magically appear between
versions. Since we are messing with some of the lower-level constructs
of the language, there are certain things that can be mildly annoying,
such as the fact that the standard library is more or less treated as
an "all or nothing" module. Recently, it has been divided up a little
bit, which helped us add data structures that depend on the heap. Now,
we can use the `alloc` crate to set a custom allocator that Rust can
use to allocate and deallocate memory. In a perfect world, we would
also be able to do something similar with a `thread` crate. I hope
this can be revisited when Rust has matured a bit. However, these
things are not high up on the Rust agenda for things to be implemented
in the language. They will get around to it, but things can move
slowly in Rust land. Async IO was a years-long debacle capped by a
ridiculously heated debate about syntax among other things. However,
the positives of the language are something to behold. Rust does not
allow data races, as it either panics or does not compile. I will
touch on the differences between these two behaviors later. Long story
short, you cannot do something very tricky to get around this unless
you go into `unsafe` mode, which we do a bit of already. As noted
above Rust is a "Systems Programming Language". The definition of a
"Systems Programming Language" is usually subject to a lot debate, but
more or less you can say something along the lines of: Performance is
important, memory layout should be predictable, and everything isn't
hidden by layers upon layers of (sometimes unnecessary)
abstractions. Rust allows users to specify how structs are laid out in
memory, and calls to malloc and free are implicit. Rust is very big on
the RAII (Resource Allocation is Initialization) way of doing
things. Instead of having a dedicated destructor like C++, Rust will
`drop` (C++ `free`) objects once they go out of lexical scope. This is
a little more complicated with Reference Counted Pointers(RCs) and
Atomically Reference Counted Pointers(ARCs), but you will not often
find yourself using those structures.

Rust also has the `asm!` macro for inline assembly. It is a goal of
Rust to nicely with C and C++. There are a few tools that allow easy
play between to the languages. One of these tools is
`bindgen`. `bindgen` allows you to feed it a C/C++ header file, and it
will generate Rust bindings, so you can call the Rust function, and it
will use the underlying implementation of the C function. There is
also a tool call `cbindgen` that will go the other direction. There is
currently a script that does the translation in Nautilus, though it
needs to be tested more.  I have been quite impressed with its
performance. It will handle unions, and bitfields as well. The
documentation is sufficient as well. However, I haven't done a lot of
testing in this region, so your mileage may vary. Outside of writing
Rust code, there are a few more hoops we had to jump through to get
things to compile. One of these to add a file under the `.cargo`
directory in the `src` directory called `config` which specifies that
when we build this binary, we do not want the redzone disabled. The
other feature that is in there now is `-Ctarget-feature=-see`. I would
assume that there may need to be more additions to this file, but it
is certainly something that is not used in mainstream Rust
development, so documentation and examples may be hard to find. That
being said, the documentation for Rust tends to be excellent. There is
also very lively discord and irc channels, where you can ask such
questions. I think personally it would be really cool to get threads
working in Nautilus. Currently, we can spawn NK threads in Nautilus,
but we are largely sidestepping the `thread` module in the standard
library and doing things ad-hoc.

## Rust Things
This will serve as a quick overview to using Rust. Rust has a very
nice build tool and package manager called `cargo`. The compiler
itself is called `rustc`. `rustup` is Rust's toolchain manager. To
build a Rust project, run `cargo build` in a directory either at the
level of the `Cargo.toml` or below it. The `Cargo.toml` file specifies
flags that should be passed to the `rustc` as well as specifies
dependencies to other Rust modules as various other project
metadata. Our `Cargo.toml` is noticeably spartan, but then again we
are not trying to write a web application with a ton of module
dependencies. Another line to notice in the `Cargo.toml` is the line
saying

``` Markdown
	[lib]
	crate-type = ["staticlib"]
```

This line specifies that the output of this project will be a .a file,
i.e. a static library also known as a bunch of .o files wrapped into
one file. Nautilus is then linked against this, so we can call into
Rust land from Nautilus.


`rustup` is what allows use to pick which version of Rust you wish to
use for a certain project. This can be specified over the command line
by using this utility. We currently use 1.36 nightly targeting
Linux. This has worked with no issues so far, but this may not
continue. 

We had to do some black magic with `.cargo/config` to pass some flags
to `rustc` that you cannot pass with `cargo`. This is an area that may
be interesting to look into. 

There are a ton of tutorials online about Rust, but the best resource
is the [Book](https://doc.rust-lang.org/book/). The other good
resource is [The
Rustonomicon](https://doc.rust-lang.org/nomicon/). This will give a
little more detail into the language implementation and how you can
sometimes bend the rules. The FFI section was really helpful in
finding out how to go back and forth between C and Rust functions. 

## Rust libc
Rust libc provides Rust bindings to C libc functions. As of this
writing, Nautilus does not have all C libc functions. We actually just
use it for the C types that are provided by rust libc. This is also
why we have to add the `--ctypes-prefix` to rust bindgen. To be able
to use rust libc, we must add a dependency to it in the `Cargo.toml`
in the `src` directory. It is incredibly important to specify that the
`default-features` is false. If this is set to true, Rust libc will
assume it can give Rust bindings that use Rust's std lib, which we do
not support. 

## Rust Bindgen Script
There is a script called `generate_bindings.sh` that lives
(tentatively) at under the `nautilus/rust_bindings` directory. There
is nothing really to the script. It is listed in it entirety with
annotations below.

``` bash
	#!/bin/bash
	# Iterate over all files in the include dirs, i.e. all the .h files
	for filename in ~/nautilus/include/nautilus/*; do
		f=$(basename $filename)
		f=${f%.*}
		 # call to bindgen, the interesting things here are the call
		 to usage of --ctypes-prefix. The libc module was described
		 above and this is just making sure we will be binding to the
		 correct types. Bindgen may kick out something like uint32_t
		 and we want to actually use ::libc::uint32_t, as we don't
		 
		 # --use-core is limited bindgen to things that are in the
		 core library of rust, i.e. there is nothing pulled from the
		stdlib
		# --builtins is used for certain builtin definitions. This may
		not be needed. 
		# -I and -include are just like their normal counterparts
		# This script should be refined as sometimes we do get double
		includes
		# The '--' is where we can begin to pass llvm compiler flags
		to bindgen
		bindgen --ctypes-prefix ::libc --use-core --builtins $filename -o ./bindings/$f"_bindings.rs" -- -I../include/ -I../include/nautilus/ -include stdint.h -include ../include/nautilus/naut_types.h
done

```

## Rust Usage
To call from Rust into Nautilus functions, we must go through the
Foreign Function Interface. In Rust, this is pretty straight
forward. I will now be walking through an example of calling into the
function `nk_vc_printf`. All of the relevant Rust code lives in `nautilus/src/rust/example/src`.

First, we must spawn the correct Rust prototype for the C
function. This can be done easily enough with bindgen. From here, you
must just thread the arguments through to the function. In this
example, `nk_vc_printf`'s prototype lives in `bindings.rs`. However,
we can bring it into scope using `use bindings::*`. This case uses the
fact that Rust files can be treated as modules and imported, if they
are declared up front in the crate root (either main.rs or
lib.rs). There is much more detail to this in the module chapter of
the Rust book.

From here, we can directly call into the C function. It is important
to note that strings are not null-terminated in Rust, and, as such,
must have a null-byte(slash followed by a 0, the actual bytestring
seems to crash my document compiler) appended to the end. There are a
few libraries that fix this task, but they often need stdlib.

For example, to actually use `nk_vc_printf`, the call to the function
looks as follows:

``` 
	// We must take the string slice below cast to a pointer
	// as then cast to the type of pointer expected by the call
	// to the generated function.
	nk_vc_printf("string_to_print\0".as_ptr() as *const i8);
```

Furthermore, all calls into such `C` functions must be wrapped in
unsafes because they are indeed unsafe, that is, you can get seg
faults or other errors that Rust avoids when you use such
functions. Rust will happily let you call into a `C` function that
dereferences a null pointer, and it will fail spectactularly I am sure.

The Rust function `nk_rust_nk_dev_dump_devices()` calls
into numerous NK C functions.

## Improvements
The file `thread.rs` was my attempt at shoving forcing NK thread to
work with Rust implementation of threads. `thread.rs` will become
much, much easier to implement when Rust starts splitting up the
stdlib, which should start happening. As mentioned earlier, the
splitting of the `alloc` crate off allowed us to use data structures
that allocate on the heap. The script will likely need some tuning as
well, but it was able to give working bindings for the modules that I
was interested in porting.

# imag - [imag-pim.org](http://imag-pim.org)

`imag` is a commandline personal information management suite.

[![Build Status](https://travis-ci.org/matthiasbeyer/imag.svg?branch=master)](https://travis-ci.org/matthiasbeyer/imag)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/imag/badge/pr?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/imag)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/imag/badge/issue?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/imag)
[![license](https://img.shields.io/github/license/matthiasbeyer/imag.svg?maxAge=2592000?style=flat-square)]()

**This application is in early development. There are _some_ things that work,
but we do not consider anything stable or usable at this moment. Feel free to
play around anyways.**

## Goal / What is imag?

Our (long-term) goal is to

> Create a fast, reliable commandline personal
> information management suite which covers all aspects of personal information
> management, consists of reusable parts and integrates well with known
> commandline tools.

imag is a PIM _helper_. We do not actually implement the PIM functionality, but
try to interface with existing PIM tools (via their API or via some standard
format they use, e.g. vcard) to make the data they manage _linkable_
and _queryable_ in an uniform way.

imag consists of _modules_ (e.g. `imag-notes`, `imag-diary`), where each module
covers one PIM aspect.
The initial approach is to use one PIM tool for one module.
So you can use `imag-todo` with [taskwarrior](https://taskwarrior.org/)
but `imag-calendar` with [icalendar](https://en.wikipedia.org/wiki/ICalendar)
files.

## Building/Running

Here is how to try `imag` out.

`imag` is a _suite/collection_ of tools and you can build them individually.
All subdirectories prefixed with "`libimag"` are libraries for the respective
binaries.
All subdirectories prefixed with `"imag-"` are binaries and compiling them will
give you a commandline application.

### Building

We use `make` to automate the build process (as `cargo` is not (yet?) able to
build several applications at once).
Make sure to _not_ include some `-j 8` arguments, as cargo parallelizes the
build process on its own. If you parallelize it with make, you end up with a
really high load on your system.

There are several targets for each of the sub-crates in the Makefile:

| Target   | Multi | Purpose                                  | Example         |
| :---     | ----- | :---                                     | :---            |
| all      |       | Build everything, debug mode             | `make all`      |
| bin      |       | Build all binaries, debug mode           | `make bin`      |
| lib      |       | Build all libraries, debug mode          | `make lib`      |
| lib-test |       | Test all libraries                       | `make lib-test` |
| imag-bin |       | Build only the `imag` binary, debug mode | `make imag-bin` |
| check    | *     | Run `cargo check`                        | `make check`    |
| clean    | *     | Remove build artifacts                   | `make clean`    |
| install  | *     | Build everything, release mode, install  | `make install`  |
| release  | *     | Build everything, release mode           | `make release`  |
| update   | *     | Run `cargo update`                       | `make update`   |

The `Multi` targets are callable for each sub-crate. For example you can call
`make imag-bookmark-check` to run `cargo check` on the `imag-bookmark` subcrate.

### Running

After you build the module you want to play with, you can simply call the binary
itself with the `--help` flag, to get some help what the module is capable of.

If you installed the module, you can either call `imag-<modulename>` (if the
install-directory is in your `$PATH`), or install the `imag` binary to call `imag
<modulename>` (also if everything is in your `$PATH`).

## Staying up-to-date

We have a [official website for imag](http://imag-pim.org), where I post
[release notes](http://imag-pim.org/releases/).
There is no RSS feed, though.

We also have a [mailinglist](http://imag-pim.org/mailinglist/) where I post
updates and where discussion and questions are encouraged.

There is a blog series which gets a update every other week
on my blog, where
[entries are tagged "imag"](http://beyermatthias.de/tags/imag.html).
I also post non-regular posts about imag things there.

I also post these blog posts
[on reddit](https://www.reddit.com/r/rust/search?q=What%27s+coming+up+in+imag&restrict_sr=on)
and submit them to [this-week-in-rust](https://this-week-in-rust.org/).

## Documentation

This is a hobby project, so sometimes things are not optimal and might go
unrecognized and slip through. Feel free to open issues about things you notice!

Though, we have some documentation in [the ./doc subtree](./doc/)
which can be compiled to PDF or a website.
These docs are not published anywhere and are not even integrated into our CI,
so it might be broken (though it's unlikely).
Developer documentation is also available
[online on github.io](https://matthiasbeyer.github.io/imag/imag_documentation/index.html).

## Please contribute!

We are looking for contributors!

There is always a number of
[complexity/easy tagged issues](https://github.com/matthiasbeyer/imag/issues?q=is%3Aopen+is%3Aissue+label%3Acomplexity%2Feasy)
available in the issue tracker you can start with and we are open to questions!

Feel free to open issues for asking questions, suggesting features or other
things!

Also have a look at [the CONTRIBUTING.md file](./CONTRIBUTING.md)!

## Contact

Have a look at [our website](http://imag-pim.org) where you can find some
information on how to get in touch and so on.

Feel free to join our new IRC channel at freenode: #imag
or our [mailinglist](http://imag-pim.org/mailinglist/).

## License

We chose to distribute this software under terms of GNU LGPLv2.1.


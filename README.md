# Rust Befunge-93

This is a toy implementation of a [Befunge](https://en.wikipedia.org/wiki/Befunge) compiler.

I implemented this as part of an old [Angel Hack challenge](http://angelhack.com/solve-these-developer-challenges-to-snag-free-tickets-to-our-series/) (though I think the bufenge challenge code on this page contains an error...).

Right now the compiler is missing two of the most important instructions in the Bufenge language - `g` and `p` which allow mutation of the running language. The program is also hard coded right now to be a random number generator.

In the future, maybe I'll add the missing `g` and `p` instructions and allow a simple command line interface so that we can run programs from files (or simple programs from the command line directly). But probably not - this was just to have some fun with Rust. Why does Befunge exist anyway?
# rlox

`rlox` is a rust implementation of an interpreter for the Lox language
used in the [Crafting Interpreters][ci] book.  I decided that following
along and using C (like the book does) would be too easy (heh), and I
wanted to each myself rust, so... here we are.

**NB**: The book is an in-progress work, and the chapters that teach by
writing a bytecode interpreter (rather than the tree-walk interpreter of
the early chapters) haven't been written/released by the author yet.  So
for now I'm writing this version with a tree-walk interpreter, but will
update it later when the bytecode-interpreter chapters get written (and
it might be interesting to compare performance at that point).

Note that I'm in the process of learning rust, so I might be doing
things in a dumb/naive way.

## Usage

### Building

Install rust (I'm using 1.22.0-beta.2) using `rustup` or your favorite
other distribution.  Then run:

```
cargo build
```

### Running

(Note: this part doesn't work yet.)

To run the REPL, just run:

```
cargo run
```

(Note: this works, but I've barely implemented support for much of
anything so far.)

To run a `.lox` script, run:

```
cargo run /path/to/script.lox
```

## Thanks

Just wanted to give a quick note of thanks to Bob Nystrom, the author of
Crafting Interpreters.  I always thought PL and compiler/interpreter
development was a scary black art.  While I do think it's still
something of a black art, I'm beginning to see it's not all that scary.
His book is amazingly accessible and well written.  I'm looking forward
to the rest of the chapters, and highly recommend [reading it][ci] if
you're at all interested in this topic.

[ci]: https://www.craftinginterpreters.com/

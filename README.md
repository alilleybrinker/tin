# tin

Tin is a _super-alpha_ (read: does not currently work) programming language,
being written for personal fun to learn how to write a programming language.
It is a true hobbyist project with no particular goal as a project other than
trying and learning new things.

Tin the _language_ does have a few design decisions already made:

* Tin will have __algebraic data types__ (because after Rust and Haskell, I
  (Andrew) struggle to live without them).
* Tin will be __Just-In-Time (JIT) compiled__ into bytecode (called "Foil")
  that runs on a virtual machine (to learn JIT compilation and profile-guided
  optimization).
* Tin will be __garbage collected__ (to learn about garbage collection
  algorithms).

Besides these decisions, things are pretty up for grabs! If you're interested
in learning to write a programming language as well, this may be a good
project for you too!

## Project Structure

Tin is split into a collection of crates, many of which are empty at the
moment. Reusable elements are split out into `libraries/`, while CLI tools
are in `tools/`.

Currently-planned CLI tools are (in rough order of expected implementation):

1. `tinc`: The Tin compiler, parsing Tin, type-checking it, optimizing it,
   and generating Foil bytecode.
2. `foilvm`: The VM which runs compiled bytecode.
3. `tint`: The Tin Tool, for creating new Tin projects, compiling and running
   programs, and more (think: `cargo` in Rust).
4. `tinenv`: The Tin installation manager, will work similarly to `pyenv` and
   `rbenv` for managing multiple installs of Tin.
5. `tinfmt`: Automatic formatter for Tin, so people can argue less about how
   code should be written.

## Current Work

Right now, the #1 task is defining the initial syntax of Tin, and implementing
the parser and HIR (High-level Intermediate Representation) of a Tin program.
This representation will be the basis for initial optimization.

Once that's done, the next two questions will be:

* Implementing type-checking. (How do we want types to be checked? Is the HIR
  the right level of abstraction or should it be translated?)
* Define the Foil bytecode language, then implement translation from HIR (or,
  if added, any mid-level intermediate representation) to Foil, along with
  the Foil VM to execute the bytecode.
* Implement the garbage collector, so Tin programs don't just continuously
  leak memory.

At this point, Tin would be 0.1, as you could actually write programs that
compile and run.

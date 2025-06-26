# parking-game-fuzzer

This is a template for a fuzzer for [parking-game](https://github.com/addisoncrump/parking-game) puzzles.
While originally built as a homework assignment, this is released publicly for folks looking to learn how to implement
their own components in [LibAFL](https://github.com/AFLplusplus/LibAFL).

## Purpose

This repo serves as examples for implementing various components of LibAFL yourself, as well as working through some of
the logic you may need to consider in your own applications.
This targets a puzzle game, for which fuzzing is almost certainly a poor application, but the game highlights certain
design patterns that crop up in the testing of real programs.
Notably, this repo teaches you how to:

- Build custom executors (including integration with custom observer types)
- Build custom observers
- Build custom feedbacks which inspect the results of those observers and interact with user statistics
- Build custom input types
- Build custom mutators for those input types
- Basic snapshot fuzzing strategies and mutators specialized for them

Obviously, since we are working with a game, the components are somewhat contrived; in real applications, you will face
other difficulties specific to your target.
This is just around to help people get their bearings with the design of LibAFL and why we implement things the way we
do, and the challenges therein are to give a sense of how to implement new components.

### Anti-Purpose

This repo is **not** designed to teach you how to fuzz real programs.
The execution and feedback mechanisms here are deeply contrived and do not represent optimal strategies.
This strictly serves as a way to think about building your own LibAFL components, not fuzzing as a whole.

### Intended Audience

This is intended for practitioners who are at least already aware of fuzzing, have a basic knowledge of Rust, and are
looking to build custom fuzzing components with LibAFL.
For folks wanting to learn fuzzing strategies for specific applications, consider reading
the [Fuzzing Book](https://www.fuzzingbook.org/) or trying to apply existing LibAFL executors.

## License

The code within this repository is licensed under CC0.
Reuse how you wish, but please give credit back as possible.

The maps provided in the [maps](maps/) directory are ported
from [Jim Storer's personal page](https://www.cs.brandeis.edu/~storer/JimPuzzles/ZPAGES/zzzTokyoParking.html).
I could not find an associated license.
Redistribute at your own risk.
The maps are distributed here as educational materials for non-commercial use.

## Exercises

TODO

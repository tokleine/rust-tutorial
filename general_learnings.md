# Learnings

- ``Vec`` are variable length (stored in heap) and ``Array`` are fixed length (stored in stack)

## Principles

- ``Box deallocation principle (fully correct): If a variable owns a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.``
- ``Moved heap data principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.``
- ``Pointer Safety Principle: data should never be aliased and mutated at the same time.``
- In sum, if a value does not own heap data, then it can be copied without a move. For example:
  - An ``i32`` does not own heap data, so it can be copied without a move.
  - A ``String`` does own heap data, so it can not be copied without a move.
  - An ``&String`` does not own heap data, so it can be copied without a move.
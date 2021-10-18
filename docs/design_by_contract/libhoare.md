# Libhoare

Simple Rust support for design by contract-style assertions. Supports

* preconditions (`precond`),
* postconditions (`postcond`),
* invariants (pre and post)  (`invariant`).

Each macro takes a predicate given as a string parameter. Each macro is
available in a `debug_` version which only checks the assertion in debug builds,
they should be zero overhead in non-debug builds. You can use `result` inside a
postcondition to get the value returned by the function.

- Preconditions are checked on entry to a function. 
- Postconditions are checked when leaving the function by any path.

Rust sample

```rust
#[precond="x > 0"]
#[postcond="result > 1"]
fn foo(x: int) -> int {
    let y = 45 / x;
    y + 1
}

struct Bar {
    f1: int,
    f2: int
}

#[invariant="x.f1 < x.f2"]
fn baz(x: &mut Bar) {
    x.f1 += 10;
    x.f2 += 10;
}
```

Rust Debug sample

```rust
#![feature(plugin, custom_attribute)]
#![plugin(hoare)]

#[debug_precond="false"]
fn test_precondition_ignored_for_release() {
    
}
```
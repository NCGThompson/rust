// run-rustfix

#![deny(rust_2021_incompatible_closure_captures)]
//~^ NOTE: the lint level is defined here
#![feature(rustc_attrs)]
#![allow(unused)]

    use std::sync::Mutex;

struct InsignificantDropPoint {
    x: i32,
    y: Mutex<i32>,
}

impl Drop for InsignificantDropPoint {
    #[rustc_insignificant_dtor]
    fn drop(&mut self) {}
}

struct SigDrop;

impl Drop for SigDrop {
    fn drop(&mut self) {}
}

struct GenericStruct<T>(T, T);

struct Wrapper<T>(GenericStruct<T>, i32);

impl<T> Drop for GenericStruct<T> {
    #[rustc_insignificant_dtor]
    fn drop(&mut self) {}
}

// Test no migration because InsignificantDropPoint is marked as insignificant
fn insign_dtor() {
    let t = (
        InsignificantDropPoint { x: 0, y: Mutex::new(0) },
        InsignificantDropPoint { x: 0, y: Mutex::new(0) }
    );

    let c = || t.0;

}

// `SigDrop` implements drop and therefore needs to be migrated.
fn significant_drop_needs_migration() {
    let t = (SigDrop {}, SigDrop {});

    let c = || {
        //~^ ERROR: drop order
        //~| NOTE: for more information, see
        //~| HELP: add a dummy let to cause `t` to be fully captured
        let _t = t.0;
        //~^ NOTE: in Rust 2018, this closure captures all of `t`, but in Rust 2021, it will only capture `t.0`
    };

    c();
}
//~^ NOTE: in Rust 2018, `t` is dropped here, but in Rust 2021, only `t.0` will be dropped here as part of the closure

// Even if a type implements an insignificant drop, if it's
// elements have a significant drop then the overall type is
// consdered to have an significant drop. Since the elements
// of `GenericStruct` implement drop, migration is required.
fn generic_struct_with_significant_drop_needs_migration() {
    let t = Wrapper(GenericStruct(SigDrop {}, SigDrop {}), 5);

    // move is used to force i32 to be copied instead of being a ref
    let c = move || {
        //~^ ERROR: drop order
        //~| NOTE: for more information, see
        //~| HELP: add a dummy let to cause `t` to be fully captured
        let _t = t.1;
        //~^ NOTE: in Rust 2018, this closure captures all of `t`, but in Rust 2021, it will only capture `t.1`
    };

    c();
}
//~^ NOTE: in Rust 2018, `t` is dropped here, but in Rust 2021, only `t.1` will be dropped here as part of the closure

fn main() {
    significant_drop_needs_migration();
    generic_struct_with_significant_drop_needs_migration();
}

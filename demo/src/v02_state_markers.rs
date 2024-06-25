//! This module represents an advanced implementation of the typestate pattern using zero-sized enums
//! to mark the state of a resource.
//!
//! Each state is represented by an un-instantiable enum.
//! The main type is generic over these 'enum markers' so information of the main type is shared
//! across states.
//!
//! The state machine implemented:
//!
#![doc = include_str!("../assets/door-2.svg")]
//!
//! # Discussion
//!
//! This implementation requires quite an understanding of Rust's type system. However, if
//! information is to be shared between all possible states this implementation allows for easier
//! access of this shared information, e.g. see [`color()`](Door::color). As all state markers are
//! un-instantiable this also means that the main type, independent of its state, has always the
//! same type which should allow the compiler to optimize for more data locality when transitioning.
//!
//! If state-specific information is required the implementation in [`crate::v03_generic_state`]
//! is a better fit.
//! Note that the implementation of `crate::v03_generic_state` is mutual exclusive to the
//! implementation in this module.
//!
//! ## On state marker traits
//!
//! Many implementations like this or the one from [`crate::v03_generic_state`] use a marker trait
//! to group all valid states, e.g.:
//!
//! ```
//! trait OpenState {}
//!
//! impl OpenState for Open {}
//! impl OpenState for Closed {}
//!
//! struct DoorRestricted<S: OpenState> // { ... }
//! # ;
//! ```
//!
//! On one hand, this may improve readability and make it more obvious how types are related. On the
//! other hand the valid states are restricted by constructors and transitions. It is open to the
//! implementor whether to add this additional code or not.

use std::marker::PhantomData;

/// The main type of the typestate pattern representing the resource that changes according to the
/// modeled state machine.
///
/// The type is generic over the state `S` so we can implement different methods depending on the
/// instantiation of `S`.
pub struct Door<S> {
    /// States are enums without variants. This means they can't be instantiated. Therefore, we need
    /// to wrap the state in [`PhantomData`].
    ///
    /// `PhantomData` is a marker type from the `std` crate that tells rustc 'assume there is an `S`
    /// stored inside this struct. This is required to tell rustc how it should apply ownership and
    /// borrowing rules.
    // todo: Remove the `state` field to see the error message you get without it.
    state: PhantomData<S>,
    color: &'static str,
}

/// enums without variants are a trick to enforce that a type can only be used at the type level and
/// not at runtime.
pub enum Open {}
pub enum Closed {}

/// `impl` blocks for generic types can be restrict the generic to various degrees:
/// 1. The generic can be instantiated by a concrete type, making all defined methods only available
///    when the generic is instantiated with this type.
///    This is used in this implementation to, e.g. restrict the [`look_through()`](Door::look_through)
///    method to the [`Open`] state.
/// 2. Limit the generic to certain trait bounds.
///    This can be useful if several states are grouped by a trait.
/// 3. No restrictions on the generic type. This allows to call defined methods independent of the
///    generic's instantiation. An example for this is the [`color()`](Door::color) method.
impl Door<Open> {
    pub fn look_through(&self) {
        println!("It's brighter on the other side!");
    }

    pub fn close(self) -> Door<Closed> {
        Door {
            // NOTE: PhantomData doesn't require an instance to be created.
            state: PhantomData,
            color: self.color,
        }
    }
}

impl Door<Closed> {
    pub fn knock(&self) {
        println!("knock knock");
    }

    pub fn open(self) -> Door<Open> {
        Door {
            state: PhantomData,
            color: self.color,
        }
    }
}

impl<S> Door<S> {
    pub fn color(&self) -> &str {
        &self.color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_look_through_a_closed_door() {
        let door = Door {
            state: PhantomData,
            color: "brown",
        };

        door.look_through();

        // let closed = door.close();

        // closed.look_through();
    }
}

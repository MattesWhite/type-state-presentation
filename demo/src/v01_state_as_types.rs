//! This is module represents the simplest implementation of the typestate pattern.
//!
//! Each state is represented by its own type.
//! Methods of the state machine can only be called in their associated states as they are only
//! implemented for them.
//! State transitions are always done via methods that take the current state by ownership (`self`)
//! and return another state type.
//! This ensures that a past state is no longer available after a transition happened.
//! With this the unit test from the [`crate::v00_runtime`] example doesn't compile with this
//! implementation.
//!
//! The state machine implemented:
//!
#![doc = include_str!("../assets/door.svg")]
//!
//! # Discussion
//!
//! This implementation is quite simple and is a good fit when the states don't share a lot of
//! information, e.g. as it is the case for transformation and validation pipelines.
//!
//! However, once information needs to be shared between states the implementations of
//! [`crate::v02_state_markers`] or [`crate::v03_generic_state`] might be a better fit.

pub struct OpenDoor;

pub struct ClosedDoor;

impl OpenDoor {
    /// An associated method only available in the `open` state.
    pub fn look_through(&self) {
        println!("It's brighter on the other side!");
    }

    /// A transition method.
    ///
    /// Note: The `OpenDoor` is consumed by ownership and the new state is returned.
    pub fn close(self) -> ClosedDoor {
        ClosedDoor
    }
}

impl ClosedDoor {
    pub fn knock(&self) {
        println!("knock knock");
    }

    pub fn open(self) -> OpenDoor {
        OpenDoor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_look_through_a_closed_door() {
        let door = OpenDoor;

        door.look_through();

        // todo: uncomment the following lines to see that the typestate pattern turns this invalid
        // method call into a compile-time error opposed to the runtime-error in `v00_runtime`.

        // let closed = OpenDoor.close();
        // closed.look_through();
    }
}

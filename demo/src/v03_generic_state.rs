//! This module represents an advanced implementation of the typestate pattern using structs to
//! represent the state of a resource.
//!
//! Each state is represented by a struct so each state can bring its own information.
//!
//! The state machine implemented:
//!
#![doc = include_str!("../assets/door-3.svg")]
//!
//! # Discussion
//!
//! This implementation is more simple than the implementation in [`crate::v02_state_markers`] as
//! it doesn't use the un-instantiable enum trick. However, as the type structs can be of different
//! size the different instantiations of the main type can also wary in size potentially making
//! transitions more costly. A way to navigate this is to put the state in a [`Box`] so independent
//! of the state struct's size the main type is always of the same size.

pub struct Door<S> {
    /// No [`PhantomData`](std::marker::PhantomData) required as the state is stored directly.
    state: S,
    color: &'static str,
}

pub struct Open {
    /// With this implementation states can provide specific information.
    angle: f64,
}

pub struct Closed;

impl Door<Open> {
    pub fn look_through(&self) {
        println!("It's brighter on the other side!");
    }

    pub fn close(self) -> Door<Closed> {
        Door {
            state: Closed,
            color: self.color,
        }
    }

    /// State specific methods can access information only available in this state.
    pub fn open_angle(&self) -> f64 {
        self.state.angle
    }
}

impl Door<Closed> {
    pub fn knock(&self) {
        println!("knock knock");
    }

    pub fn open(self) -> Door<Open> {
        Door {
            state: Open { angle: 90.0 },
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
            state: Open { angle: 45.0 },
            color: "brown",
        };

        door.look_through();

        // let closed = door.close();

        // closed.look_through();
    }
}

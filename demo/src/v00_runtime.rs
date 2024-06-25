//! This is not an implementation of the typestate pattern but the base implementation that relies
//! on runtime checks.
//!
//! The state machine implemented:
//!
#![doc = include_str!("../assets/door.svg")]
//!
//! # Discussion
//!
//! This implementation is quite simple and most programmers coming from an OOP language should
//! understand it.
//! However, when you run the unit tests you will notice that the test will fail as the compiler
//! can't know that the [`look_through()`](Door::look_through()) method doesn't work when the door
//! is closed.

pub struct Door {
    is_open: bool,
}

impl Door {
    pub fn look_through(&self) {
        if self.is_open {
            println!("It's brighter on the other side!");
        } else {
            panic!("Can't look through a closed door");
        }
    }

    pub fn knock(&self) {
        if self.is_open {
            panic!("Where should I knock on?");
        } else {
            println!("knock knock");
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_look_through_a_closed_door() {
        let mut door = Door { is_open: true };

        door.look_through();

        door.close();

        door.look_through();
    }
}

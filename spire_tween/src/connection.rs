/*
use super::*;

pub struct Connection {
    pub callable: Callable,
    pub flags: Flags,
}

impl Connection {
    /// Returns true if the connection should be retained, false if it should be removed.
    pub fn invoke(&self) -> bool {
        if self.flags.contains(Flags::ConnectDeferred) {
            self.callable.call_deferred(&[]);
        } else {
            self.callable.call(&[]);
        }

        !self.flags.contains(Flags::ConnectOneShot)
    }
}
*/

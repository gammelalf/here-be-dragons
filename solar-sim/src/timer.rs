//! System and resource to track the time passed between dispatches

use std::ops::Deref;
use std::time::Duration;

use specs::{System, Write};

#[rustfmt::skip]
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use wasm_timer::Instant;

/// Resource of time passed since last dispatch
///
/// Updated by [`Timer`] system
#[derive(Copy, Clone, Debug)]
pub struct Delta(Duration);

impl Default for Delta {
    fn default() -> Self {
        Self(Duration::from_secs(0))
    }
}

impl Deref for Delta {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// System tracking time passed between dispatches
///
/// Updates [`Delta`] resource
#[derive(Copy, Clone, Debug)]
pub struct Timer(Instant);

impl Default for Timer {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl<'a> System<'a> for Timer {
    type SystemData = Write<'a, Delta>;

    fn run(&mut self, mut delta: Self::SystemData) {
        let last = self.0;
        self.0 = Instant::now();
        delta.0 = self.0.duration_since(last);
    }
}

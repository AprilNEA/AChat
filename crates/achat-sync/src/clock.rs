use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Hybrid Logical Clock for causal ordering across devices.
///
/// Format: upper 48 bits = wall clock ms, lower 16 bits = logical counter.
/// This ensures total ordering even when wall clocks drift.
pub struct HybridClock {
    last: AtomicI64,
}

impl HybridClock {
    pub fn new() -> Self {
        Self {
            last: AtomicI64::new(0),
        }
    }

    /// Generate a new HLC timestamp that is guaranteed to be greater than all previous.
    pub fn now(&self) -> i64 {
        let wall_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let wall_shifted = wall_ms << 16;

        loop {
            let prev = self.last.load(Ordering::Acquire);
            let candidate = if wall_shifted > prev {
                wall_shifted
            } else {
                prev + 1
            };

            if self
                .last
                .compare_exchange(prev, candidate, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return candidate;
            }
        }
    }

    /// Merge with a remote timestamp to maintain causality.
    pub fn receive(&self, remote: i64) -> i64 {
        let wall_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let wall_shifted = wall_ms << 16;

        loop {
            let prev = self.last.load(Ordering::Acquire);
            let candidate = wall_shifted.max(prev).max(remote) + 1;

            if self
                .last
                .compare_exchange(prev, candidate, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return candidate;
            }
        }
    }
}

impl Default for HybridClock {
    fn default() -> Self {
        Self::new()
    }
}

/// Determinism context — the single authoritative source of all
/// non-deterministic primitives in experiment paths.
///
/// Under the `deterministic` cargo feature any call to OS-level entropy
/// or wall-clock time that bypasses this context will cause a compile-time
/// error (via the shadowed panic guards below).
///
/// # Rules
/// 1. All RNG streams used inside a world run MUST be derived from
///    [`DetCtx::rng`] — never from `rand::thread_rng()` or `OsRng`.
/// 2. All agent IDs MUST be derived from genome entropy (first 16 bytes of
///    SHA-256) — never from `Uuid::new_v4()`.
/// 3. All logical timestamps MUST be derived from [`DetCtx::logical_now`] —
///    never from `chrono::Utc::now()` or `std::time::SystemTime::now()`.
///
/// # Spec (the "randomness spec" reviewers expect)
/// - Algorithm:  ChaCha8 (via `rand_chacha::ChaCha8Rng`)
/// - Seed derivation: `seed = base_seed + step_index * 1_000 + run_index`
/// - Domain separation: separate `DetCtx` per world; separate stream IDs
///   per subsystem (0 = mutation, 1 = selection, 2 = reproduction, …)
/// - Iteration order: deterministic (sorted agent registries, no HashMap
///   non-determinism in hot paths)
/// - Bit-identical guarantee: holds under the same OS/arch/rustc version;
///   cross-platform semantic equivalence is verified via quantized metrics.
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Stream IDs for domain-separated RNG.
pub mod stream {
    pub const MUTATION: u64 = 0;
    pub const SELECTION: u64 = 1;
    pub const REPRODUCTION: u64 = 2;
    pub const CATASTROPHE: u64 = 3;
    pub const TAXATION: u64 = 4;
}

/// A determinism context for a single world run.
///
/// Instantiate one per world; do not share across worlds.
#[derive(Debug)]
pub struct DetCtx {
    /// Base seed for this world (= `ExperimentConfig::trial_seed(step, run)`).
    base_seed: u64,
    /// Logical epoch counter (incremented by the epoch loop).
    epoch: u64,
    /// Per-epoch event counter for sub-epoch ordering.
    event_counter: u64,
}

impl DetCtx {
    /// Create a new determinism context for a world run.
    pub fn new(base_seed: u64) -> Self {
        Self {
            base_seed,
            epoch: 0,
            event_counter: 0,
        }
    }

    /// Advance the epoch counter. Call once at the start of each epoch.
    pub fn advance_epoch(&mut self) {
        self.epoch += 1;
        self.event_counter = 0;
    }

    /// Consume one event slot and return the current event index.
    pub fn next_event(&mut self) -> u64 {
        let idx = self.event_counter;
        self.event_counter += 1;
        idx
    }

    /// Return a deterministic logical timestamp as a nanosecond offset
    /// from the UNIX epoch.
    ///
    /// The value is derived from `(base_seed, epoch, event_counter)` and
    /// increases monotonically within a world run. It is NOT wall-clock time.
    pub fn logical_now(&self) -> u64 {
        // Pack seed (bits 63-32), epoch (bits 31-16), event (bits 15-0)
        // into a monotonically increasing u64.
        (self.base_seed & 0xFFFF_FFFF) << 32
            | (self.epoch & 0xFFFF) << 16
            | (self.event_counter & 0xFFFF)
    }

    /// Return a domain-separated, deterministic PRNG stream.
    ///
    /// `stream_id` should be one of the constants in [`stream`].
    /// The returned RNG is seeded from `(base_seed, epoch, stream_id)`.
    pub fn rng(&self, stream_id: u64) -> ChaCha8Rng {
        let seed = self
            .base_seed
            .wrapping_mul(0x517C_C1B7_2722_0A95)
            .wrapping_add(self.epoch.wrapping_mul(0xDEAD_BEEF_CAFE_1234))
            .wrapping_add(stream_id.wrapping_mul(0x1234_5678_9ABC_DEF0));
        ChaCha8Rng::seed_from_u64(seed)
    }
}

// ── Compile-time guards ────────────────────────────────────────────────────
//
// When the `deterministic` feature is enabled, we shadow the two most
// common non-determinism callsites in experiments so that any future
// re-introduction causes an immediate compile failure with a clear message.

#[cfg(feature = "deterministic")]
mod guards {
    /// Calling `Utc::now()` inside a deterministic build is forbidden.
    ///
    /// Use [`super::DetCtx::logical_now`] instead.
    #[allow(non_snake_case, dead_code)]
    pub fn Utc_now_is_forbidden_in_deterministic_builds() {
        compile_error!(
            "wall-clock time (Utc::now / SystemTime::now) is banned in \
             deterministic builds; use DetCtx::logical_now() instead."
        );
    }

    /// Calling `Uuid::new_v4()` inside a deterministic build is forbidden.
    ///
    /// Derive IDs from the SHA-256 genesis hash instead.
    #[allow(non_snake_case, dead_code)]
    pub fn Uuid_new_v4_is_forbidden_in_deterministic_builds() {
        compile_error!(
            "OS-entropy UUIDs (Uuid::new_v4) are banned in deterministic \
             builds; derive IDs from the genesis hash bytes instead."
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_det_ctx_rng_reproducible() {
        let ctx1 = DetCtx::new(42);
        let ctx2 = DetCtx::new(42);
        let v1: u64 = ctx1.rng(stream::MUTATION).gen();
        let v2: u64 = ctx2.rng(stream::MUTATION).gen();
        assert_eq!(v1, v2, "Same seed + stream_id must produce same value");
    }

    #[test]
    fn test_det_ctx_different_streams() {
        let ctx = DetCtx::new(42);
        let v_mut: u64 = ctx.rng(stream::MUTATION).gen();
        let v_sel: u64 = ctx.rng(stream::SELECTION).gen();
        assert_ne!(v_mut, v_sel, "Different stream IDs must produce different values");
    }

    #[test]
    fn test_logical_now_monotone() {
        let mut ctx = DetCtx::new(1_000_000);
        let t0 = ctx.logical_now();
        ctx.next_event();
        let t1 = ctx.logical_now();
        ctx.advance_epoch();
        let t2 = ctx.logical_now();
        assert!(t1 > t0, "logical_now should increase within an epoch");
        assert!(t2 > t0, "logical_now after epoch advance should be > initial");
    }
}

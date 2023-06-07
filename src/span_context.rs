use std::cell::RefCell;
use std::time::{Duration, Instant};
use std::fmt;
use std::hash::Hash;
use std::num::ParseIntError;
use rand::Rng;

// pub struct SpanContext {
//     pub duration: Duration,
//     pub last_timestamp: Instant,
// }

// use crate::error::{TraceResult, TraceError};

/// Helper to create span ids for testing
impl SpanId {
    pub fn _from_u64(num: u64) -> Self {
        SpanId::from_bytes(num.to_be_bytes())
    }
}


/// An 8-byte value which identifies a given span.
///
/// The id is valid if it contains at least one non-zero byte.
#[derive(Clone, PartialEq, Eq, Copy, Hash)]
pub struct SpanId(u64);

impl SpanId {
    /// Invalid span id
    pub const INVALID: SpanId = SpanId(0);

    /// Create a span id from its representation as a byte array.
    pub const fn from_bytes(bytes: [u8; 8]) -> Self {
        SpanId(u64::from_be_bytes(bytes))
    }

    /// Return the representation of this span id as a byte array.
    pub const fn _to_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    /// Converts a string in base 16 to a span id.
    pub fn from_hex(hex: &str) -> Result<Self, ParseIntError> {
        u64::from_str_radix(hex, 16).map(SpanId)
    }

    pub fn rand() -> Self {
        CURRENT_RNG.with(|rng| SpanId::from(rng.borrow_mut().gen::<[u8; 8]>()))
    }
}

impl From<[u8; 8]> for SpanId {
    fn from(bytes: [u8; 8]) -> Self {
        SpanId::from_bytes(bytes)
    }
}

impl fmt::Debug for SpanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:016x}", self.0))
    }
}

impl fmt::Display for SpanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:016x}", self.0))
    }
}

impl fmt::LowerHex for SpanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}



/// Immutable portion of a [`Span`] which can be serialized and propagated.
///
/// This representation conforms to the [W3C TraceContext specification].
///
/// Spans that do not have the `sampled` flag set in their [`TraceFlags`] will
/// be ignored by most tracing tools.
///
/// [`Span`]: crate::trace::Span
/// [W3C TraceContext specification]: https://www.w3.org/TR/trace-context
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct SpanContext {
    // pub trace_id: TraceId,
    // pub span_id: SpanId,
    // pub trace_flags: TraceFlags,
    // pub is_remote: bool,
    // pub trace_state: TraceState,
    pub duration: Duration,
    pub last_timestamp: Instant,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn span_id_test_data() -> Vec<(SpanId, &'static str, [u8; 8])> {
        vec![
            (SpanId(0), "0000000000000000", [0, 0, 0, 0, 0, 0, 0, 0]),
            (SpanId(42), "000000000000002a", [0, 0, 0, 0, 0, 0, 0, 42]),
            (SpanId(5508496025762705295), "4c721bf33e3caf8f", [76, 114, 27, 243, 62, 60, 175, 143])
        ]
    }
   

    #[test]
    fn test_span_id() {
        for test_case in span_id_test_data() {
            assert_eq!(format!("{}", test_case.0), test_case.1);
            assert_eq!(format!("{:016x}", test_case.0), test_case.1);
            assert_eq!(test_case.0._to_bytes(), test_case.2);

            assert_eq!(test_case.0, SpanId::from_hex(test_case.1).unwrap());
            assert_eq!(test_case.0, SpanId::from_bytes(test_case.2));
        }
    }

}


thread_local! {
    /// Store random number generator for each thread
    static CURRENT_RNG: std::cell::RefCell<rand::rngs::ThreadRng>  = RefCell::new(rand::rngs::ThreadRng::default());
}
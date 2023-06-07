use std::{fmt, num::ParseIntError, cell::RefCell};

use rand::Rng;


use crate::{span_context::SpanId};

pub const TRACEPARENT_HEADER: &str = "traceparent";
pub const SUPPORTED_VERSION: u8 = 0;

/// A 16-byte value which identifies a given trace.
///
/// The id is valid if it contains at least one non-zero byte.
#[derive(Clone, PartialEq, Eq, Copy, Hash)]
pub struct TraceId(u128);

impl TraceId {
    /// Invalid trace id
    pub const INVALID: TraceId = TraceId(0);

    /// Create a trace id from its representation as a byte array.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        TraceId(u128::from_be_bytes(bytes))
    }

    /// Return the representation of this trace id as a byte array.
    pub const fn to_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    pub fn from_hex(hex: &str) -> Result<Self, ParseIntError> {
        u128::from_str_radix(hex, 16).map(TraceId)
    }

    pub fn rand() -> Self {
        CURRENT_RNG.with(|rng| TraceId::from(rng.borrow_mut().gen::<[u8; 16]>()))
    }
}

impl From<[u8; 16]> for TraceId {
    fn from(bytes: [u8; 16]) -> Self {
        TraceId::from_bytes(bytes)
    }
}

impl fmt::Debug for TraceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:032x}", self.0))
    }
}

impl fmt::Display for TraceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:032x}", self.0))
    }
}

impl fmt::LowerHex for TraceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

/// Helper to create trace ids for testing
impl TraceId {
    pub fn _from_u128(num: u128) -> Self {
        TraceId::from_bytes(num.to_be_bytes())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TraceContext {
    // pub tracer_id: u64,
    pub trace_id: TraceId,
    pub transaction_id: SpanId,
    pub span_id: SpanId,
    pub parent_id: Option<SpanId>,
}

impl Default for TraceContext {
    fn default() -> Self {
        TraceContext { 
            trace_id: TraceId::INVALID, 
            transaction_id: SpanId::INVALID, 
            span_id: SpanId::INVALID, 
            parent_id: None,
        }
    }
}


/// Flags that can be set on a [`SpanContext`].
///
/// The current version of the specification only supports a single flag
/// [`TraceFlags::SAMPLED`].
///
/// See the W3C TraceContext specification's [trace-flags] section for more
/// details.
///
/// [trace-flags]: https://www.w3.org/TR/trace-context/#trace-flags
#[derive(Clone, Debug, Default, PartialEq, Eq, Copy, Hash)]
pub struct TraceFlags(u8);

impl TraceFlags {
    /// Trace flags with the `sampled` flag set to `1`.
    ///
    /// Spans that are not sampled will be ignored by most tracing tools.
    /// See the `sampled` section of the [W3C TraceContext specification] for details.
    ///
    /// [W3C TraceContext specification]: https://www.w3.org/TR/trace-context/#sampled-flag
    pub const SAMPLED: TraceFlags = TraceFlags(0x01);
}


impl std::ops::BitAnd for TraceFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for TraceFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::Not for TraceFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl fmt::LowerHex for TraceFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}


thread_local! {
    /// Store random number generator for each thread
    static CURRENT_RNG: std::cell::RefCell<rand::rngs::ThreadRng>  = RefCell::new(rand::rngs::ThreadRng::default());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn trace_id_test_data() -> Vec<(TraceId, &'static str, [u8; 16])> {
        vec![
            (TraceId(0), "00000000000000000000000000000000", [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            (TraceId(42), "0000000000000000000000000000002a", [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42]),
            (TraceId(126642714606581564793456114182061442190), "5f467fe7bf42676c05e20ba4a90e448e", [95, 70, 127, 231, 191, 66, 103, 108, 5, 226, 11, 164, 169, 14, 68, 142])
        ]
    }

    #[test]
    fn test_trace_id() {
        for test_case in trace_id_test_data() {
            assert_eq!(format!("{}", test_case.0), test_case.1);
            assert_eq!(format!("{:032x}", test_case.0), test_case.1);
            assert_eq!(test_case.0.to_bytes(), test_case.2);

            // let short_trace_id = (test_case.0.0 >> 64 ) as u64;
            // println!("{}",format!("{:016x}", short_trace_id) );

            assert_eq!(test_case.0, TraceId::from_hex(test_case.1).unwrap());
            assert_eq!(test_case.0, TraceId::from_bytes(test_case.2));
        }
    }
}
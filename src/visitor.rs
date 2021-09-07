use std::fmt::Debug;

use fxhash::FxHashMap;
use serde::Serialize;
use serde_json::{json, Value};
use tracing::field::{Field, Visit};

use crate::config::TRACE_ID_FIELD_NAME;

#[derive(Default)]
#[repr(transparent)]
pub(crate) struct ApmVisitor(pub(crate) FxHashMap<String, Value>);

impl Visit for ApmVisitor {
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.insert_value(field, value);
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.insert_value(field, value);
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.insert_value(field, value);
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.insert_value(field, value);
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        self.insert_value(field, format!("{:?}", value));
    }
}

impl ApmVisitor {
    #[inline]
    fn insert_value<T>(&mut self, field: &Field, value: T)
    where
        T: Serialize,
    {
        self.0.insert(field.name().to_string(), json!(value));
    }
}

#[derive(Default)]
#[repr(transparent)]
pub(crate) struct TraceIdVisitor(pub(crate) Option<u64>);

impl Visit for TraceIdVisitor {
    fn record_i64(&mut self, _field: &Field, _value: i64) {}

    fn record_u64(&mut self, field: &Field, value: u64) {
        if field.name() == TRACE_ID_FIELD_NAME {
            self.0 = Some(value);
        }
    }

    fn record_bool(&mut self, _field: &Field, _value: bool) {}

    fn record_str(&mut self, _field: &Field, _value: &str) {}

    fn record_debug(&mut self, _field: &Field, _value: &dyn Debug) {}
}

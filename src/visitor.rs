use std::fmt::Debug;

use fxhash::FxHashMap;
use serde::Serialize;
use serde_json::{json, Value};
use tracing::field::{Field, Visit};

#[derive(Default)]
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

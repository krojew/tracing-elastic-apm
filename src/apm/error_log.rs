use rand::{thread_rng, Rng};
use tracing_core::{Event};

use crate::{trace_context::TraceContext, visitor::ApmVisitor, apm::*};




pub(crate) fn create_error_log(visitor: &mut ApmVisitor,event: &Event<'_>, trace_ctx: &TraceContext) -> model::Error{

    let metadata = event.metadata();

    let error_log = crate::apm::model::Error {
        id: thread_rng().gen::<u128>().to_string(),
        trace_id: Some(trace_ctx.trace_id.to_string()),
        parent_id: Some(trace_ctx.span_id.to_string()),
        culprit: Some(metadata.target().to_string()),
        log: Some(crate::apm::model::Log {
            level: Some(metadata.level().to_string()),
            message: fields::message(visitor),
            ..Default::default()
        }),
        ..Default::default()
    };
    error_log
}
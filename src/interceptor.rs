use crate::{
    span_ext::ApmSpanExt, trace_context::{SUPPORTED_VERSION, TRACEPARENT_HEADER, TraceFlags}};

#[derive(Clone)]
pub struct TonicTraceInterceptor;

impl tonic::service::Interceptor for TonicTraceInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> tonic::Result<tonic::Request<()>> {
        let ctx = tracing::Span::current().context();
        let header_value = format!(
            "{:02x}-{:032x}-{:016x}-{:02x}",
            SUPPORTED_VERSION,
            ctx.trace_id,
            ctx.span_id,
            TraceFlags::SAMPLED
        );
        let metadata = req.metadata_mut();
        metadata.insert(TRACEPARENT_HEADER, header_value.parse().unwrap());
        Ok(req)
    }
}

pub fn tonic_tracing_intercept(mut req: tonic::Request<()>) -> tonic::Result<tonic::Request<()>> {
    let ctx = tracing::Span::current().context();

    let header_value = format!(
        "{:02x}-{:032x}-{:016x}-{:02x}",
        SUPPORTED_VERSION,
        ctx.trace_id,
        ctx.span_id,
        TraceFlags::SAMPLED
    );
    let metadata = req.metadata_mut();
    metadata.insert(TRACEPARENT_HEADER, header_value.parse().unwrap());
    Ok(req)
} 
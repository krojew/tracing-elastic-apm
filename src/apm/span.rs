

use fxhash::FxHashMap;

use crate::{trace_context::TraceContext,apm::*, visitor::ApmVisitor};

pub(crate) fn create_span(timestamp:u64,name: &String, visitor: &mut ApmVisitor, trace_ctx: &TraceContext) -> model::Span{
    let span_type = fields::span_type(visitor);
    let sub_type =  fields::span_subtype(visitor);
    let destination = fields::destination(visitor);
    let db = fields::db(visitor);
    let http =  fields::http(visitor);

    let new_span = model::Span {
        id: trace_ctx.span_id.to_string(),
        trace_id: trace_ctx.trace_id.to_string(),
        parent_id: trace_ctx.parent_id.unwrap().to_string(),
        timestamp: Some(timestamp),
        name: fields::span_name(visitor).unwrap_or(name.clone()),
        span_type: span_type,
        subtype: sub_type,
        context: Some(model::SpanContext{
            destination: destination,
            db: db,
            http: http,
            tags: None,
            // service: None,
            message: None,
            ..Default::default()
        }),
        ..Default::default()
    };
    new_span
}


pub(crate) fn close_span(span:&mut model::Span, visitor:&mut ApmVisitor,_trace_ctx: &TraceContext) {

    let http_status_code =  fields::http_status_code(visitor);
    // let http_result =  fields::http_result(visitor);
    let http_headers:Option<FxHashMap<String, String>> = fields::http_response_headers(visitor);
    span.outcome = match fields::span_outcome(visitor) {
        Some(outcome) => {
            if outcome == "success" {
                Some(model::Outcome::Success)
            }else {
                Some(model::Outcome::Failure)
            }
        },
        None => Some(model::Outcome::Unknown),
    };
    if let Some(context) = &mut span.context {
        if let Some(http) = &mut context.http {

            http.status_code = http_status_code;

            if let Some(resp) = &mut http.response {
                resp.status_code = http_status_code;
                resp.headers = http_headers;
            }
            match http_status_code {
                Some(code) => {
                    if code >= 400 {
                        span.outcome = Some(model::Outcome::Failure);
                    }else {
                        span.outcome = Some(model::Outcome::Success);
                    }
                },
                None => todo!(),
            }
            
            
        }
    }
    
}
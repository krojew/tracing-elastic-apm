
use fxhash::FxHashMap;
use crate::{trace_context::TraceContext, visitor::ApmVisitor, apm::*};

pub(crate) fn create_transaction(timestamp:u64,name: &String, visitor: &mut ApmVisitor, trace_ctx: &TraceContext) -> model::Transaction{
    let new_transaction = model::Transaction {
        id: trace_ctx.span_id.to_string(),
        transaction_type: fields::span_type(visitor),
        subtype: fields::span_subtype(visitor),
        trace_id: trace_ctx.trace_id.to_string(),
        timestamp: Some(timestamp),
        name: Some( fields::span_name(visitor).unwrap_or(name.clone()) ),
        context: Some(model::TransactionContext{
            response: Some(model::Response{
                ..Default::default()
            }),
            request: Some(model::Request{
                body: None,
                headers: fields::http_request_headers(visitor),
                http_version: fields::http_version(visitor),
                method: fields::http_method(visitor).unwrap_or("unknown".to_string()),
                socket: None,
                url: model::Url{
                    full: fields::http_url(visitor),
                    protocol: fields::http_schema(visitor),
                    hostname: fields::http_host(visitor),
                    pathname: fields::http_pathname(visitor),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    new_transaction
}

pub(crate) fn close_transaction(transaction:&mut model::Transaction, visitor:&mut ApmVisitor,_trace_ctx: &TraceContext) {
    let http_status_code =  fields::http_status_code(visitor);
    let http_headers:Option<FxHashMap<String, String>> = fields::http_response_headers(visitor);
    transaction.result = fields::span_result(visitor);
    transaction.outcome = match fields::span_outcome(visitor) {
        Some(outcome) => {
            if outcome == "success" {
                Some(model::Outcome::Success)
            }else {
                Some(model::Outcome::Failure)
            }
        },
        None => Some(model::Outcome::Unknown),
    };
    if let Some(context) = &mut transaction.context {
        if let Some(resp) = &mut context.response {
            resp.status_code = http_status_code;
            resp.headers = http_headers;
            match http_status_code {
                Some(code) => {
                    if code >= 400 {
                        transaction.outcome = Some(model::Outcome::Failure);
                    }else {
                        transaction.outcome = Some(model::Outcome::Success);
                    }
                },
                None => ()
            }
        }
    }
    
}
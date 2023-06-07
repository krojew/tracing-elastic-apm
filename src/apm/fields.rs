use fxhash::FxHashMap;
use serde_json::Value;

use crate::span_context::SpanId;
use crate::trace_context::{TraceId, TRACEPARENT_HEADER};
use crate::visitor::ApmVisitor;

use crate::apm::*;

pub(crate) fn message(visitor: &mut ApmVisitor) -> String {
    visitor.0.remove("message").map( |v| v.as_str().unwrap().to_string()).unwrap_or_default()
}

pub(crate) fn span_name(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("span.name").map( |v| v.as_str().unwrap().to_string())
}

pub(crate) fn span_type(visitor: &mut ApmVisitor) -> String {
    visitor.0.remove("span.span_type").map_or("request".to_owned(), |v| v.as_str().unwrap().to_string())
}

pub(crate) fn span_subtype(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("span.sub_type").map(|v| v.as_str().unwrap().to_string())
}

pub(crate) fn span_outcome(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("span.outcome").map(|v| v.as_str().unwrap().to_string())
}

pub(crate) fn span_result(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("span.result").map(|x| x.as_str().unwrap().to_string())
}

pub(crate) fn destination(visitor: &mut ApmVisitor) -> Option<model::Destination> {
    match visitor.0.remove("destination.service.name") {
        Some(service_name) => {
            Some(model::Destination{
                address: visitor.0.get("destination.address").map(|x| x.as_str().unwrap().to_string()),
                port: visitor.0.get("destination.port").map(|x| x.as_i64().unwrap() as i32),
                service: Some(model::DestinationService{
                    service_type: destination_service_type(visitor),
                    name: service_name.as_str().unwrap().to_string(),
                    ..Default::default()
                }),
            })
        }
        None => None
    }
}


pub(crate) fn destination_service_type(visitor: &mut ApmVisitor) -> String {
    visitor.0.remove("destination.service_type").map(|x| x.as_str().unwrap().to_string()).unwrap_or("external".to_string())
}


pub(crate) fn db(visitor: &mut ApmVisitor) -> Option<model::Db> {
    match visitor.0.remove("db.db_type") {
        Some(db_type) =>  {
            Some(model::Db {
                instance: db_instance(visitor),
                link: db_link(visitor),
                statement: db_statement(visitor),
                db_type: Some(db_type.as_str().unwrap().to_string()),
                user: db_user(visitor),
                rows_affected: db_row_affected(visitor),
            })
        }
        None => None
    }
}

pub(crate) fn db_instance(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("db.instance").map(|x| x.as_str().unwrap().to_string())
}

pub(crate) fn db_link(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("db.link").map(|x| x.as_str().unwrap().to_string())
}

pub(crate) fn db_statement(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("db.statement").map(|x| x.as_str().unwrap().to_string())
}

pub(crate) fn db_user(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("db.user").map(|x| x.as_str().unwrap().to_string())
}

pub(crate) fn db_row_affected(visitor: &mut ApmVisitor) -> Option<i32> {
    visitor.0.remove("db.rows_affected").map(|x| x.as_i64().unwrap() as i32)
}

pub(crate) fn http(visitor: &mut ApmVisitor) -> Option<model::Http> {
    match visitor.0.remove("http.url") {
        Some(_) =>  {
            let http_status_code = http_status_code(visitor);
            Some(model::Http {
                url: http_url(visitor),
                status_code: http_status_code,
                method: http_method(visitor),
                response: Some(model::Response{
                    status_code: http_status_code,
                    transfer_size: None,
                    encoded_body_size: None,
                    decoded_body_size: None,
                    headers: http_response_headers(visitor),
                }),
            })
        }
        None => None
    }
}
pub(crate) fn http_url(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("http.url").map(|x| x.as_str().unwrap().to_string())
}

pub(crate) fn http_schema(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("http.schema").map(|x| x.as_str().unwrap().to_string())
}

pub(crate) fn http_host(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("http.host").map(|x| x.as_str().unwrap().to_string())
}

pub(crate) fn http_pathname(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("http.pathname").map(|x| x.as_str().unwrap().to_string())
}

pub(crate) fn extract_http_pathname(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.get("http.pathname").map(|x| x.as_str().unwrap().to_string())
}

pub(crate) fn http_version(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("http.version").map(|x| x.as_str().unwrap().to_string())
}


pub(crate) fn http_method(visitor: &mut ApmVisitor) -> Option<String> {
    visitor.0.remove("http.method").map(|x| x.as_str().unwrap().to_string())
}

pub(crate) fn http_status_code(visitor: &mut ApmVisitor) -> Option<i32> {
    visitor.0.remove("http.status_code").map(|x| x.as_i64().unwrap() as i32)
}

pub(crate) fn http_request_headers(visitor: &mut ApmVisitor) -> Option<FxHashMap<String, String>> { 
    match visitor.0.remove("http.request.headers") {
        Some(v) => {
            let str = v.as_str().unwrap();
            let parsed: Value = serde_json::from_str(str).unwrap();
            match  parsed.as_object() {
                Some(m) => {
                    let mut result = FxHashMap::default();
    
                    for (k,v) in m {
                        result.insert(k.to_string(), v.as_str().unwrap().to_string());
                    }
                    
                    Some(result)
                }
                None => None
            }
        }
        None => None
    }
}

pub(crate) fn http_response_headers(visitor: &mut ApmVisitor) -> Option<FxHashMap<String, String>> { 
    match visitor.0.remove("http.response.headers") {
        Some(v) => {
            let str = v.as_str().unwrap();
            let parsed: Value = serde_json::from_str(str).unwrap();
            match  parsed.as_object() {
                Some(m) => {
                    let mut result = FxHashMap::default();
    
                    for (k,v) in m {
                        result.insert(k.to_string(), v.as_str().unwrap().to_string());
                    }
                    
                    Some(result)
                }
                None => None
            }
        }
        None => None
    }
}


pub(crate) fn extract_traceparent(visitor: &mut ApmVisitor) -> Option<(TraceId,SpanId)> {
    match visitor.0.get("http.request.headers") {
        Some(v) => {
            let str = v.as_str().unwrap();
            let parsed: serde_json::Value = serde_json::from_str(str).unwrap();
            match  parsed.as_object() {
                Some(m) => {
                    for (key,value) in m {
                        if key == TRACEPARENT_HEADER {
                           return _extract_traceparent(value.as_str().unwrap_or_default());
                        }
                    }
                    None
                }
                None => None
            }
        }
        None => None
    }
}

fn _extract_traceparent(traceparent: &str) -> Option<(TraceId,SpanId)> {
    let parts = traceparent.split_terminator('-').collect::<Vec<&str>>();
    // Ensure parts are not out of range.
    if parts.len() < 4 {
        return None
    }
    if let Ok(trace_id) = TraceId::from_hex(parts[1]){
        if let Ok(parent_id) = SpanId::from_hex(parts[2]) {
            return Some((trace_id,parent_id));
        }
    }
    
    None
}
use axum::{
    extract::{ConnectInfo, MatchedPath, OriginalUri},
    response::Response,
};
use http::{header, uri::Scheme, HeaderMap, Method, Request, Version, HeaderValue};
use tracing_core::field::Empty;
use std::{borrow::Cow, net::SocketAddr, time::Duration};
use tower_http::{
    classify::{
        GrpcErrorsAsFailures, GrpcFailureClass, ServerErrorsAsFailures, ServerErrorsFailureClass,
        SharedClassifier,
    },
    trace::{MakeSpan, OnBodyChunk, OnEos, OnFailure, OnRequest, OnResponse, TraceLayer},
};
use tracing::{Span};

use crate::{span_ext::ApmSpanExt, trace_context::{TraceFlags, SUPPORTED_VERSION, TRACEPARENT_HEADER} };


pub fn apm_tracing_layer() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    ApmMakeSpan,
    ApmOnRequest,
    ApmOnResponse,
    ApmOnBodyChunk,
    ApmOnEos,
    ApmOnFailure,
> {
    TraceLayer::new_for_http()
        .make_span_with(ApmMakeSpan)
        .on_request(ApmOnRequest)
        .on_response(ApmOnResponse)
        .on_body_chunk(ApmOnBodyChunk)
        .on_eos(ApmOnEos)
        .on_failure(ApmOnFailure)
}

/// OpenTelemetry tracing middleware for gRPC.
pub fn apm_tracing_layer_grpc() -> TraceLayer<
    SharedClassifier<GrpcErrorsAsFailures>,
    ApmMakeGrpcSpan,
    ApmOnRequest,
    ApmOnResponse,
    ApmOnBodyChunk,
    ApmOnEos,
    ApmOnGrpcFailure,
> {
    TraceLayer::new_for_grpc()
        .make_span_with(ApmMakeGrpcSpan)
        .on_request(ApmOnRequest)
        .on_response(ApmOnResponse)
        .on_body_chunk(ApmOnBodyChunk)
        .on_eos(ApmOnEos)
        .on_failure(ApmOnGrpcFailure)
}

/// A [`MakeSpan`] that creates tracing spans using [OpenTelemetry's conventional field names][otel].
///
/// [otel]: https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/semantic_conventions/http.md
#[derive(Clone, Copy, Debug)]
pub struct ApmMakeSpan;

impl<B> MakeSpan<B> for ApmMakeSpan {
    fn make_span(&mut self, req: &Request<B>) -> Span {
        let user_agent = req
            .headers()
            .get(header::USER_AGENT)
            .map_or("", |h| h.to_str().unwrap_or(""));

        let host = req
            .headers()
            .get(header::HOST)
            .map_or("", |h| h.to_str().unwrap_or(""));

        let scheme = req
            .uri()
            .scheme()
            .map_or_else(|| "HTTP".into(), http_scheme);

        let _http_route = req
            .extensions()
            .get::<MatchedPath>()
            .map_or_else(|| "", |mp| mp.as_str())
            .to_owned();

        let uri = if let Some(uri) = req.extensions().get::<OriginalUri>() {
            uri.0.clone()
        } else {
            req.uri().clone()
        };
        let _http_target = uri
            .path_and_query()
            .map(|path_and_query| path_and_query.to_string())
            .unwrap_or_else(|| uri.path().to_owned());

        let http_path = uri.path();

        let client_ip = parse_x_forwarded_for(req.headers())
            .or_else(|| {
                req.extensions()
                    .get::<ConnectInfo<SocketAddr>>()
                    .map(|ConnectInfo(client_ip)| Cow::from(client_ip.to_string()))
            })
            .unwrap_or_default();
        let http_method_v = http_method(req.method());
        let name = format!("{http_method_v} {http_path}").trim().to_string();
        let http_headers = format!("{:?}",req.headers());
        let span_type = "request".to_string();
        let span_subtype = "http".to_string();
        let url: String = uri.to_string();
        let span = tracing::info_span!(
            "HTTP request",
            span.name= %name,
            span.span_type = %span_type,
            span.subtype = %span_subtype,

            client.ip = %client_ip,
            http.version = %http_flavor(req.version()),
            http.request.headers = %http_headers,
            http.url = %url,
            http.host = %host,
            http.method = %http_method_v,
            http.scheme = %scheme,
            // http.target = %http_target,
            http.user_agent = %user_agent,
            http.pathname = %http_path,

            http.status_code = Empty,
            http.response.headers = Empty,
        );
        span
    }
}

/// A [`MakeSpan`] that creates tracing spans using [OpenTelemetry's conventional field names][otel] for gRPC services.
///
/// [otel]: https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/semantic_conventions/http.md
#[derive(Clone, Copy, Debug)]
pub struct ApmMakeGrpcSpan;

impl<B> MakeSpan<B> for ApmMakeGrpcSpan {
    fn make_span(&mut self, req: &Request<B>) -> Span {
        let user_agent = req
            .headers()
            .get(header::USER_AGENT)
            .map_or("", |h| h.to_str().unwrap_or(""));

        let host = req
            .headers()
            .get(header::HOST)
            .map_or("", |h| h.to_str().unwrap_or(""));

        let scheme = req
            .uri()
            .scheme()
            .map_or_else(|| "HTTP".into(), http_scheme);

        let _http_route = req
            .extensions()
            .get::<MatchedPath>()
            .map_or("", |mp| mp.as_str())
            .to_owned();

        let uri = if let Some(uri) = req.extensions().get::<OriginalUri>() {
            uri.0.clone()
        } else {
            req.uri().clone()
        };
        let http_target = uri
            .path_and_query()
            .map(|path_and_query| path_and_query.to_string())
            .unwrap_or_else(|| uri.path().to_owned());

        let client_ip = parse_x_forwarded_for(req.headers())
            .or_else(|| {
                req.extensions()
                    .get::<ConnectInfo<SocketAddr>>()
                    .map(|ConnectInfo(client_ip)| Cow::from(client_ip.to_string()))
            })
            .unwrap_or_default();
        let http_method_v = http_method(req.method());
        let http_headers = format!("{:?}",req.headers());
        let span_type = "request".to_string();
        let url: String = req.uri().to_string();
        let span_subtype = "grpc".to_string();
        let span = tracing::info_span!(
            "GRPC request",
            span.name = %http_target, // Convetion in gRPC tracing.
            span.span_type = %span_type,
            span.sub_type = %span_subtype,
            client.ip = %client_ip,
            http.version = %http_flavor(req.version()),
            http.host = %host,
            http.method = %http_method_v,
            http.request.headers = %http_headers,
            http.url = %url,
            http.scheme = %scheme,
            http.user_agent = %user_agent,
            http.pathname = %http_target,

            http.status_code = Empty,
            http.response.headers = Empty,
        );
        span
    }
}

fn parse_x_forwarded_for(headers: &HeaderMap) -> Option<Cow<'_, str>> {
    let value = headers.get("x-forwarded-for")?;
    let value = value.to_str().ok()?;
    let mut ips = value.split(',');
    Some(ips.next()?.trim().into())
}

fn http_method(method: &Method) -> Cow<'static, str> {
    match method {
        &Method::CONNECT => "CONNECT".into(),
        &Method::DELETE => "DELETE".into(),
        &Method::GET => "GET".into(),
        &Method::HEAD => "HEAD".into(),
        &Method::OPTIONS => "OPTIONS".into(),
        &Method::PATCH => "PATCH".into(),
        &Method::POST => "POST".into(),
        &Method::PUT => "PUT".into(),
        &Method::TRACE => "TRACE".into(),
        other => other.to_string().into(),
    }
}

fn http_flavor(version: Version) -> Cow<'static, str> {
    match version {
        Version::HTTP_09 => "0.9".into(),
        Version::HTTP_10 => "1.0".into(),
        Version::HTTP_11 => "1.1".into(),
        Version::HTTP_2 => "2.0".into(),
        Version::HTTP_3 => "3.0".into(),
        other => format!("{other:?}").into(),
    }
}

fn http_scheme(scheme: &Scheme) -> Cow<'static, str> {
    if scheme == &Scheme::HTTP {
        "http".into()
    } else if scheme == &Scheme::HTTPS {
        "https".into()
    } else {
        scheme.to_string().into()
    }
}


/// Callback that [`Trace`] will call when it receives a request.
///
/// [`Trace`]: tower_http::trace::Trace
#[derive(Clone, Copy, Debug)]
pub struct ApmOnRequest;

impl<B> OnRequest<B> for ApmOnRequest {
    #[inline]
    fn on_request(&mut self, _request: &Request<B>, _span: &Span) { }
}

/// Callback that [`Trace`] will call when it receives a response.
///
/// [`Trace`]: tower_http::trace::Trace
#[derive(Clone, Copy, Debug)]
pub struct ApmOnResponse;

impl<B> OnResponse<B> for ApmOnResponse {
    fn on_response(self, response: &Response<B>, _latency: Duration, span: &Span) {
        let http_headers = format!("{:?}",response.headers());
        let status = response.status().as_u16();
        span.record("http.status_code", status);
        span.record("http.response.headers", http_headers);
    }
}

/// Callback that [`Trace`] will call when the response body produces a chunk.
///
/// [`Trace`]: tower_http::trace::Trace
#[derive(Clone, Copy, Debug)]
pub struct ApmOnBodyChunk;

impl<B> OnBodyChunk<B> for ApmOnBodyChunk {
    #[inline]
    fn on_body_chunk(&mut self, _chunk: &B, _latency: Duration, _span: &Span) {}
}

/// Callback that [`Trace`] will call when a streaming response completes.
///
/// [`Trace`]: tower_http::trace::Trace
#[derive(Clone, Copy, Debug)]
pub struct ApmOnEos;

impl OnEos for ApmOnEos {
    #[inline]
    fn on_eos(self, _trailers: Option<&http::HeaderMap>, _stream_duration: Duration, _span: &Span) {}
}

/// Callback that [`Trace`] will call when a response or end-of-stream is classified as a failure.
///
/// [`Trace`]: tower_http::trace::Trace
#[derive(Clone, Copy, Debug)]
pub struct ApmOnFailure;

impl OnFailure<ServerErrorsFailureClass> for ApmOnFailure {
    fn on_failure(&mut self, failure: ServerErrorsFailureClass, _latency: Duration, span: &Span) {
        match failure {
            ServerErrorsFailureClass::StatusCode(status) => {
                if status.is_server_error() {
                    span.record("otel.status_code", "ERROR");
                }
            }
            ServerErrorsFailureClass::Error(_) => {
                span.record("otel.status_code", "ERROR");
            }
        }
    }
}

/// Callback that [`Trace`] will call when a response or end-of-stream is classified as a failure.
///
/// [`Trace`]: tower_http::trace::Trace
#[derive(Clone, Copy, Debug)]
pub struct ApmOnGrpcFailure;

impl OnFailure<GrpcFailureClass> for ApmOnGrpcFailure {
    fn on_failure(&mut self, failure: GrpcFailureClass, _latency: Duration, span: &Span) {
        match failure {
            GrpcFailureClass::Code(code) => {
                span.record("http.grpc_status", code);
            }
            GrpcFailureClass::Error(_) => {
                span.record("http.grpc_status", 1);
            }
        }
    }
}


// pub fn inject_trace_context<T>(mut res: Response<T>) -> Response<T> {
//     let span = &Span::current();
//     let mut meta = span.metadata();
//     let visitor = ApmVisitor::default();

//     span.record("transparent", "transparent");
    
//     println!("inject_trace_context,span.metadata={:?}, res.headers={:?}",span.metadata(), res.headers());
//     //TraceContextPropagator::new().inject_context(&ctx, &mut HeaderInjector(res.headers_mut()));
//     res
// }

// pub fn extract_trace_context<T>(req: &Request<T>) -> Span {
//     // let context = TraceContextPropagator::new().extract(&HeaderExtractor(req.headers()));
    
//     let span = Span::current();
//     let headers = req.headers();
//     println!(">>>extract_trace_context,span={:?}",span);
//     span.extract(headers);
//     span
// }

pub fn inject_trace_context<T>(mut res: Response<T>) -> Response<T> {
    let ctx = Span::current().context();
    let header_value = format!(
        "{:02x}-{:032x}-{:016x}-{:02x}",
        SUPPORTED_VERSION,
        ctx.trace_id,
        ctx.span_id,
        TraceFlags::SAMPLED
    );
    // injector.set(TRACEPARENT_HEADER, header_value);

    match HeaderValue::from_str(header_value.as_str()) {
        Ok(v) => {
            res.headers_mut().insert(TRACEPARENT_HEADER,v);
        },
        Err(_) => (),
    }
    res
}
use crate::apm::service::Service;
use crate::apm::*;


pub fn get_exec_name() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
}


pub fn build_service() -> Option<crate::apm::service::Service> {
    let apm_environment: String = std::env::var("ELASTIC_APM_ENVIRONMENT").expect("ELASTIC_APM_ENVIRONMENT should be setting");
    let apm_service_name: String = std::env::var("ELASTIC_APM_SERVICE_NAME").unwrap_or(get_exec_name().unwrap());
    let apm_service_version: Option<String> = std::env::var("ELASTIC_APM_SERVICE_VERSION").ok();

    Some(
        Service::new(Some(apm_service_name), 
        apm_service_version,
        Some(apm_environment),
        build_language(), 
        build_runtime(), 
        build_framework(),
        build_node())
    )
}

fn build_framework() -> Option<model::Framework> {
    Some(model::Framework {
        name: Some("opentelemetry-otlp".to_owned()),
        version: None,
    })
}

fn build_language() -> Option<model::Language> {
    Some(model::Language {
        name: "rust".to_owned(),
        version: None,
    })
}

fn build_runtime() -> Option<model::Runtime> {
    Some(model::Runtime {
        name: "tokio".to_string(),
        version: "1".to_string(),
        // version: None,
    })
}

fn build_node() -> Option<model::ServiceNode> {
    Some(model::ServiceNode {
        configured_name: std::env::var("ELASTIC_APM_SERVICE_NODE_NAME").ok(),
        // version: None,
    })
}
use std::process;
use sysinfo::{ProcessExt, System, SystemExt};
pub(crate) fn build_process() -> Option<model::Process> {
    let sys = System::new_all();
    let mut process_name = None;
    let mut argv = None;
    let process_id = process::id();
    
    for (pid, process) in sys.processes() {
        
        if  pid.to_string() == process_id.to_string(){
            process_name = Some(process.name().to_string());
            argv = Some(process.cmd().into());
            break;
        }
    }
    Some(
        model::Process {
            pid: process::id() as i32, 
            ppid: None, 
            title: process_name, 
            argv: argv,
        }
    )
}


pub(crate) fn build_system() -> Option<model::System> {
    Some(
        model::System {
            architecture: Some(format!("{}",std::env::consts::ARCH)),
            hostname: Some(gethostname::gethostname().into_string().unwrap()),
            // detected_hostname: todo!(),
            // configured_hostname: todo!(),
            platform: Some(format!("{}",std::env::consts::OS)),
            // container: todo!(),
            // kubernetes: todo!(),
            ..Default::default()
        }
    )
}


use fxhash::FxHashMap;
use sysinfo::{System, SystemExt, ProcessExt, CpuExt};

use crate::apm::*;
use std::process;


const METRIC_SYSTEM_CPU_USAGE: &str = "system.cpu.total.norm.pct";
const METRIC_PROCESS_CPU_USAGE: &str = "system.process.cpu.total.norm.pct";
const METRIC_SYSTEM_MEM_TOTAL: &str = "system.memory.total";    // Total physical memory.
const METRIC_SYSTEM_MEM_FREE: &str = "system.memory.actual.free";  // Amount of memory available without swapping.
const METRIC_PROCESS_MEM_VIRTUAL: &str = "system.process.memory.size";
const METRIC_PROCESS_MEM_RESIDENT: &str = "system.process.memory.rss.bytes";


pub(crate) fn gather_metrics() -> model::Metrics {
    let mut metrics =  model::Metrics::default();
    let sys = System::new_all();
    let process_id = process::id();

    for (pid, process) in sys.processes() {
        if  pid.to_string() == process_id.to_string(){
        
            // metrics.samples.insert(value)
            let cpu_info = sys.global_cpu_info();
            let mut samples : FxHashMap<String,model::Metric> = Default::default();
            
            samples.insert(METRIC_SYSTEM_CPU_USAGE.into(), model::Metric{
                value: cpu_info.cpu_usage() as f64 / 100.0,
                ..Default::default()
            });
            samples.insert(METRIC_PROCESS_CPU_USAGE.into(), model::Metric{
                value: process.cpu_usage() as f64 / 100.0,
                ..Default::default()
            });
            samples.insert(METRIC_SYSTEM_MEM_TOTAL.into(), model::Metric{
                value: sys.total_memory() as f64,
                ..Default::default()
            });
            samples.insert(METRIC_SYSTEM_MEM_FREE.into(), model::Metric{
                value: sys.free_memory() as f64,
                ..Default::default()
            });
            samples.insert(METRIC_PROCESS_MEM_VIRTUAL.into(), model::Metric{
                value: process.virtual_memory() as f64 / 1000.0,
                ..Default::default()
            });
            samples.insert(METRIC_PROCESS_MEM_RESIDENT.into(), model::Metric{
                value: process.memory() as f64 ,
                ..Default::default()
            });

            metrics.samples = Some(samples);
            break;
        }
    }
    metrics
}


// pub(crate) fn init_metric() {
//     let mut sched = JobScheduler::new();

//     sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
//         println!("I get executed every 10 seconds!");
//     }));

//     loop {
//         sched.tick();

//         std::thread::sleep(Duration::from_millis(500));
//     }
// }

#[cfg(test)]
mod tests {
    use std::{process, fmt::Debug};

    use sysinfo::{System, SystemExt, ProcessExt, CpuExt};
    use super::*;

    #[test]
    fn test_sys_info() {
        let sys = System::new_all();
        let cpus = sys.cpus();

        println!("cpus={:?}",cpus);
        let process_id = process::id();
    
        for (pid, process) in sys.processes() {
            
            if  pid.to_string() == process_id.to_string(){
                println!("===============================");
                let cpu_info = sys.global_cpu_info();
                println!("{}            =   {:?}",METRIC_SYSTEM_CPU_USAGE,cpu_info.cpu_usage());
                println!("{}            =   {:?}",METRIC_PROCESS_CPU_USAGE,process.cpu_usage());

                
                println!("{}            =   {:?}",METRIC_SYSTEM_MEM_TOTAL,sys.total_memory() as f64 );
                println!("{}            =   {:?}",METRIC_SYSTEM_MEM_FREE,sys.free_memory() as f64);

                println!("{}            =   {:?}",METRIC_PROCESS_MEM_VIRTUAL,process.virtual_memory() as f64 / 1000.0);
                println!("{}            =   {:?}",METRIC_PROCESS_MEM_RESIDENT,process.memory() as f64 );

                println!("===============================");

                break;
            }
        }


        




    }
}
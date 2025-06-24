use sysinfo::{System, SystemExt, CpuExt};

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    println!("System Monitoring Tool");
    println!("----------------------");
    println!("Memory: {}/{} MB", 
        system.used_memory() / 1024 / 1024,
        system.total_memory() / 1024 / 1024
    );
    println!("Swap: {}/{} MB",
        system.used_swap() / 1024 / 1024,
        system.total_swap() / 1024 / 1024
    );
    println!("CPU Usage: {:.1}%", system.global_cpu_info().cpu_usage());
}

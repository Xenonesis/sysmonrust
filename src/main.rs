use sysinfo::{System, SystemExt, CpuExt, ProcessExt, CpuRefreshKind, ProcessRefreshKind, RefreshKind};
use eframe::{egui, App, Frame, NativeOptions};
use std::time::Duration;

#[derive(PartialEq)]
enum SortOrder {
    Cpu,
    Memory,
}

struct SystemMonitorApp {
    system: System,
    sort_order: SortOrder,
    last_refresh: std::time::Instant,
}

impl SystemMonitorApp {
    fn new() -> Self {
        Self {
            system: System::new_with_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::new())
                    .with_memory()
                    .with_processes(ProcessRefreshKind::new()),
            ),
            sort_order: SortOrder::Cpu,
            last_refresh: std::time::Instant::now(),
        }
    }
}

impl App for SystemMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let now = std::time::Instant::now();
        if now.duration_since(self.last_refresh) > Duration::from_secs(1) {
            self.system.refresh_all();
            self.last_refresh = now;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("System Monitoring Tool");
            ui.separator();
            ui.label(format!(
                "Memory: {}/{} MB",
                self.system.used_memory() / 1024 / 1024,
                self.system.total_memory() / 1024 / 1024
            ));
            ui.label(format!(
                "Swap: {}/{} MB",
                self.system.used_swap() / 1024 / 1024,
                self.system.total_swap() / 1024 / 1024
            ));
            ui.label(format!("CPU Usage: {:.1}%", self.system.global_cpu_info().cpu_usage()));

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.sort_order, SortOrder::Cpu, "Sort by CPU");
                ui.radio_value(&mut self.sort_order, SortOrder::Memory, "Sort by Memory");
            });

            ui.separator();
            ui.label("Processes:");
            let available_height = ui.available_height() - 50.0; // Subtract some space for other elements
            egui::ScrollArea::vertical().max_height(available_height).show(ui, |ui| {
                let mut processes: Vec<_> = self.system.processes().iter().collect();
                match self.sort_order {
                    SortOrder::Cpu => {
                        processes.sort_by(|&(_, a), &(_, b)| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
                    }
                    SortOrder::Memory => {
                        processes.sort_by(|&(_, a), &(_, b)| b.memory().cmp(&a.memory()));
                    }
                }

                for (pid, process) in processes {
                    let cpu_usage = process.cpu_usage();
                    let memory_usage_mb = process.memory() / 1024 / 1024;
                    ui.label(format!(
                        "PID: {} | Name: {} | CPU: {:.1}% | Mem: {} MB",
                        pid,
                        process.name(),
                        cpu_usage,
                        memory_usage_mb
                    ));
                }
            });
        });

        ctx.request_repaint_after(Duration::from_secs(1));
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "System Monitor",
        native_options,
        Box::new(|_cc| Box::new(SystemMonitorApp::new())),
    )
}

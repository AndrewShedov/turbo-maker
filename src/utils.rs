use sysinfo::System;

pub fn get_cpu_info() -> (usize, String) {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu_count = sys.cpus().len();
    let cpu_model = sys.cpus()[0].brand().to_string();
    (cpu_count, cpu_model)
}

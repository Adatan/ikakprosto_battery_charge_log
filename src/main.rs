use std::thread::sleep;
use std::time::Duration;
use std::fs::OpenOptions;
use winapi::um::winbase::{
    GetSystemPowerStatus,
    SYSTEM_POWER_STATUS
};
use std::io::Write;
use chrono::Local;

const CHECK_TIME: u64 = 10;
const FILENAME: &str = "battery_log.txt";

fn main() {
    let mut current_battery_life_percent = 0u8;
    let mut system_power_status = SYSTEM_POWER_STATUS::default();
    loop {
        if unsafe { GetSystemPowerStatus(&mut system_power_status) } == 1 {
            if system_power_status.BatteryLifePercent != current_battery_life_percent {
                current_battery_life_percent = system_power_status.BatteryLifePercent;
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(FILENAME)
                    .unwrap();

                file.write_all(format!("[{}]: {}%\n", Local::now().format("%d.%m.%y %H:%M").to_string(), 1).as_bytes()).unwrap();
            }
        }
        sleep(Duration::from_secs(CHECK_TIME));
    }
}
use std::thread::sleep;
use std::time::Duration;
use std::fs::OpenOptions;
use std::io::Write;
use std::mem::zeroed;
use chrono::Local;

const CHECK_TIME: u64 = 10;
const FILENAME: &str = "battery_log.txt";

fn main() {
    let mut last_battery_life_percent = 0u8;
    let mut current_battery_life_percent = 0u8;
    #[cfg(target_os = "windows")]
    let mut system_power_status = unsafe { zeroed::<winapi::um::winbase::SYSTEM_POWER_STATUS>() };
    loop {
        #[cfg(target_os = "windows")]
        {
            if unsafe { winapi::um::winbase::GetSystemPowerStatus(&mut system_power_status) } == 1 {
                current_battery_life_percent = system_power_status.BatteryLifePercent;
            }
        }

        #[cfg(target_os = "macos")] // Get current battery charge percent in MacOS
        {

        }

        #[cfg(target_os = "linux")] // Get current battery charge percent in Linux (like ubuntu)
        {

        }

        if current_battery_life_percent != last_battery_life_percent {
            last_battery_life_percent = current_battery_life_percent;
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(FILENAME)
                .unwrap();

            file.write_all(format!("[{}]: {}%\n", Local::now().format("%d.%m.%y %H:%M:%S").to_string(), last_battery_life_percent).as_bytes()).unwrap();
        }
        current_battery_life_percent = 0;
        sleep(Duration::from_secs(CHECK_TIME));
    }
}
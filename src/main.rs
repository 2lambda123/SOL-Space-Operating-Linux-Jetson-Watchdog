
extern crate difference;
mod watchdog_daemon;
mod log_daemon; // Our implementation of a dmesg watcher
use chrono::{Datelike, Timelike, Utc};

// USAGE: sudo ~/jetson-watchdog

//global check register

fn main() {

        // Generate, format and assemble the current date string in UTC
        let now = Utc::now();
        let (is_common_era, year) = now.year_ce();
        let my_timestamp = 
            format!("{}-{}-{:0>2}", 
                year.to_string(),
                now.month().to_string(),
                now.day().to_string());
        //for a timestamp that also includes time of open and AM/PM, use code below 
/*                    
        let (is_pm, hour) = now.hour12();
        let my_timestamp = 
            format!("{:0>2}:{:0>2}:{:0>2} {} {}-{}-{:0>2}", 
                hour.to_string(),
                now.minute().to_string(), 
                now.second().to_string(),
                (if is_pm { "PM" } else { "AM" }),
                year.to_string(),
                now.month().to_string(),
                now.day().to_string()); */

// TODO: Output to JSON
        // Open a log file with that name
        // let contents = fs::read_to_string(filename)
        //     .expect("Something went wrong reading the log");
        //  FIXME: add a daily check to make a new one every day?
        //  - use same one each day, have Main bark to dmesg to say it rebooted (should be obvious)

        // Discard oldest log after thirty days
        //  -best way is with linux command: 'find /var/log -name "*.json" -type f -mtime +30 -exec rm -f {} \;'



    //load config, use flight defaults if no file, or parts missing

    //wait for boot to finish

    //get baseline ps and lshw
    println!("Starting Daemon");
    
 //   watchdog_daemon::start_watchdog_daemon();

    println!("Daemon Started!!!");
    //check # resets to see aliveness, compare against previous baselines, log diffs
    //update new baseline, # resets

    //create pipe
    //start logging daemon
    log_daemon::main(); // TODO: change the main from main? Get return value from dmesg loop, be able to relaunch on failure
    //TODO: Set up channels, send the receiver over to log_daemon to communicate back
    //

//COPIED FROM LOG_DAEMON FOR REUSE WITH CHANNELS---------------------------------------------    
    // let error_type = cap.get(3).unwrap().as_str(); // take the third argument of the regex, which is the error message
    // let raw_timestamp = cap.get(1).unwrap().as_str().replace("[", "").replace("]", "").replace(" ", ""); // take the timestamp
    // let timestamp = raw_timestamp.parse::<f32>().unwrap(); // FIXME: can we process this as a string?
    // // save the timestamp on the global errors vector, then according the individual error type
    // all_errors_vec.push(timestamp);
    // match error_type { // switch-case statement for processing each error
    //     "SBE ERR" =>                {sbe_err_vec.push(timestamp);},
    //     "SError detected" =>        {serror_vec.push(timestamp);},
    //     "CPU Memory Error" =>       {cpu_mem_vec.push(timestamp);},
    //     "Machine Check Error" =>    {cce_machine_vec.push(timestamp);},
    //     "GPU L2" =>                 {gpu_l2_vec.push(timestamp);},
    //     "generated a mmu fault" =>  {mmu_fault_vec.push(timestamp);},
    //     "SDHCI_INT_DATA_TIMEOUT" => {flash_write_vec.push(timestamp);},
    //     "Timeout waiting for hardware interrupt" => {flash_read_vec.push(timestamp);},
    //     "watchdog detected" =>      {watchdog_detected_vec.push(timestamp);},
    //     _ =>                         continue, // default case
    // }
    // // DEBUG PRINTS: watch the error totals increase
    // println!("SBE ERR total: {}", sbe_err_vec.len());
    // println!("Serror total: {}", serror_vec.len());
    // println!("CPU Memory Error total: {}", cpu_mem_vec.len());
    // println!("CCE Machine Check Error total: {}", cce_machine_vec.len());
    // println!("GPU L2 Error total: {}", gpu_l2_vec.len());
    // println!("MMU Fault Error Counter: {}", mmu_fault_vec.len());
    // println!("Flash Write Error total: {}", flash_write_vec.len());
    // println!("Flash Read Error total: {}",flash_read_vec.len());
    // println!("Watchdog CPU Error total (detected): {}", watchdog_detected_vec.len());
    // println!("All errors: {}", all_errors_vec.len());
    

    //start watchdog daemon

}
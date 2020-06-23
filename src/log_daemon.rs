/* This program spawns a child process that reads from dmesg and puts dmesg lines into a buffer. 
The main thread then reads that buffer and collects timestamps for each of 9 error types into vectors of timestamps.
Known issues:  Child process kill command is UNIX only, no Windows implementation. 

A Voegtlin, SIE-3
JHU APL 6/2020
Supercomputing in Space IRAD
*/

extern crate regex;
use regex::Regex;
//for preexisting logs
// use std::env;
use std::fs; //accessing files
// use std::vec; //vectors
// -----------
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use chrono::{Datelike, Timelike, Utc};

pub fn main () {
    // Generate, format and assemble the current date-time string in UTC
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    let my_timestamp = 
        format!("{:0>2}:{:0>2}:{:0>2} {} {}-{}-{:0>2}", 
            hour.to_string(),
            now.minute().to_string(), 
            now.second().to_string(),
            (if is_pm { "PM" } else { "AM" }),
            year.to_string(),
            now.month().to_string(),
            now.day().to_string());
    // Open a log file with that name
    //  FIXME: add a daily check to make a new one every day?

    // Create empty, dynamic vectors to store error timestamps in for later processing.
    //     FIXME: Vectors cannot be left to grow without bounds. Create a fixed length and a way of saving results if the length is exceeded.
    let mut all_errors_vec: Vec<f32> = Vec::new();
    let mut sbe_err_vec: Vec<f32> = Vec::new();
    let mut serror_vec: Vec<f32> = Vec::new();
    let mut cpu_mem_vec: Vec<f32> = Vec::new(); 
    let mut cce_machine_vec: Vec<f32> = Vec::new(); 
    let mut gpu_l2_vec: Vec<f32> = Vec::new();
    let mut mmu_fault_vec: Vec<f32> = Vec::new(); 
    let mut flash_write_vec: Vec<f32> = Vec::new(); 
    let mut flash_read_vec: Vec<f32> = Vec::new(); 
    let mut watchdog_detected_vec: Vec<f32> = Vec::new(); 
    // Create the regex
    let re = Regex::new(r"(\[.?[0-9]+\.[0-9]+\])(.*?)(SBE ERR|SError detected|CPU Memory Error|Machine Check Error|GPU L2|generated a mmu fault|SDHCI_INT_DATA_TIMEOUT|Timeout waiting for hardware interrupt|watchdog detected)").unwrap();
    // Create the child process, which watches dmesg outputs change 
    let mut child = Command::new("dmesg")
        .arg("-w")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Unable to spawn dmesg child program");
// MAIN LOOP: read the child's stdout buffer forever, and process. 
// TODO: If child dies, relaunch
// TODO: Output to JSON
    while let Some(ref mut stdout) = child.stdout { // while there is something in child's stdout pipe
        let lines = BufReader::new(stdout).lines();
        for line in lines { 
            let our_string = line.unwrap().to_string(); // string type so we can run it through regex
            for cap in re.captures_iter(&our_string) {
                // println!("{}", cap.get(1).unwrap().as_str());
                // println!("{}", cap.get(2).unwrap().as_str());
                // println!("{}", cap.get(3).unwrap().as_str());
                let error_type = cap.get(3).unwrap().as_str(); // take the second argument of the regex, which is the error message
                let raw_timestamp = cap.get(1).unwrap().as_str().replace("[", "").replace("]", "").replace(" ", ""); // take the timestamp
                let timestamp = raw_timestamp.parse::<f32>().unwrap(); // FIXME: can we process this as a string?
                // save the timestamp on the global errors vector, then according the individual error type
                all_errors_vec.push(timestamp);
                match error_type { // switch-case statement for processing each error
                    "SBE ERR" =>                {sbe_err_vec.push(timestamp);},
                    "SError detected" =>        {serror_vec.push(timestamp);},
                    "CPU Memory Error" =>       {cpu_mem_vec.push(timestamp);},
                    "Machine Check Error" =>    {cce_machine_vec.push(timestamp);},
                    "GPU L2" =>                 {gpu_l2_vec.push(timestamp);},
                    "generated a mmu fault" =>  {mmu_fault_vec.push(timestamp);},
                    "SDHCI_INT_DATA_TIMEOUT" => {flash_write_vec.push(timestamp);},
                    "Timeout waiting for hardware interrupt" => {flash_read_vec.push(timestamp);},
                    "watchdog detected" =>      {watchdog_detected_vec.push(timestamp);},
                    _ =>                         continue, // default case
                }
                // DEBUG PRINTS: watch the error totals increase
                println!("SBE ERR total: {}", sbe_err_vec.len());
                println!("Serror total: {}", serror_vec.len());
                println!("CPU Memory Error total: {}", cpu_mem_vec.len());
                println!("CCE Machine Check Error total: {}", cce_machine_vec.len());
                println!("GPU L2 Error total: {}", gpu_l2_vec.len());
                println!("MMU Fault Error Counter: {}", mmu_fault_vec.len());
                println!("Flash Write Error total: {}", flash_write_vec.len());
                println!("Flash Read Error total: {}",flash_read_vec.len());
                println!("Watchdog CPU Error total (detected): {}", watchdog_detected_vec.len());
                println!("All errors: {}", all_errors_vec.len());
            }
            continue;        
        } // end of line processing
    } // end of while
    child.wait().expect("log_daemon.rs: Failed to wait on child");
}
use clap::{Parser, Subcommand};
use mini_pjt_7_rust_pilot_isl::{
    extract, load, query_create as create_data, query_delete as delete_data,
    query_read as read_data, query_update as update_data,
};
use std::time::Instant;
use sysinfo::{ProcessExt, System, SystemExt};

#[derive(Parser)]
#[command(name = "example")]
#[command(about = "A Rust program for SQLite ETL and query pipeline with memory and runtime tracking", long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "https://raw.githubusercontent.com/nogibjj/Mini_PJT_6_Complex-SQL-Query-for-a-MySQL-Database_ISL/main/data_raw/HR_1.csv"
    )]
    url: String,

    #[arg(short, long, default_value = "HR_1.csv")]
    file_path: String,

    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Extract {
        #[arg(short, long, default_value_t = 10)]
        timeout: u64,
    },
    Load,
    Create,
    Read,
    Update,
    Delete,
    All,
}

fn track_memory_and_time<F>(operation_name: &str, operation: F) -> Result<(), String>
where
    F: FnOnce() -> Result<String, String>,
{
    let start_time = Instant::now();
    let mut system = System::new_all();
    system.refresh_all();
    let pid = sysinfo::get_current_pid().expect("Failed to get process ID");
    let initial_memory = system.process(pid).map_or(0, |process| process.memory());

    // Execute the operation
    let result = operation();

    // Measure final memory usage
    system.refresh_all();
    let final_memory = system.process(pid).map_or(0, |process| process.memory());

    // Calculate elapsed time and memory usage
    let elapsed_time = start_time.elapsed();
    let memory_used = final_memory.saturating_sub(initial_memory);

    // Print metrics for the operation
    println!("{} completed in: {:.2?}", operation_name, elapsed_time);
    println!("Memory used during {}: {} KB", operation_name, memory_used);

    result.map(|_| ())
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Extract { timeout } => {
            if let Err(e) =
                track_memory_and_time("Extract", || extract(&args.url, &args.file_path, timeout))
            {
                println!("Error in extract: {}", e);
            }
        }
        Action::Load => {
            if let Err(e) = track_memory_and_time("Load", || load(&args.file_path)) {
                println!("Error in load: {}", e);
            }
        }
        Action::Create => {
            if let Err(e) = track_memory_and_time("Create", create_data) {
                println!("Error in create: {}", e);
            }
        }
        Action::Read => {
            if let Err(e) = track_memory_and_time("Read", read_data) {
                println!("Error in read: {}", e);
            }
        }
        Action::Update => {
            if let Err(e) = track_memory_and_time("Update", update_data) {
                println!("Error in update: {}", e);
            }
        }
        Action::Delete => {
            if let Err(e) = track_memory_and_time("Delete", delete_data) {
                println!("Error in delete: {}", e);
            }
        }
        Action::All => {
            // Perform all actions in sequence
            if let Err(e) =
                track_memory_and_time("Extract", || extract(&args.url, &args.file_path, 10))
            {
                println!("Error in extract: {}", e);
            }
            if let Err(e) = track_memory_and_time("Load", || load(&args.file_path)) {
                println!("Error in load: {}", e);
            }
            if let Err(e) = track_memory_and_time("Create", create_data) {
                println!("Error in create: {}", e);
            }
            if let Err(e) = track_memory_and_time("Read", read_data) {
                println!("Error in read: {}", e);
            }
            if let Err(e) = track_memory_and_time("Update", update_data) {
                println!("Error in update: {}", e);
            }
            if let Err(e) = track_memory_and_time("Delete", delete_data) {
                println!("Error in delete: {}", e);
            }
        }
    }
}

// use mini_pjt_7_rust_pilot_isl::{
//     extract, load, query_create, query_delete, query_read, query_update,
// };
// use std::time::Instant;
// use sysinfo::{ProcessExt, System, SystemExt};

// fn performance<F, R>(func: F, func_name: &str) -> Result<R, String>
// where
//     F: FnOnce() -> Result<R, String>,
// {
//     let mut sys = System::new_all();
//     sys.refresh_all();
//     let pid = sysinfo::get_current_pid().expect("Failed to get PID");

//     let start_memory = sys
//         .process(pid)
//         .expect("Failed to get current process")
//         .memory();
//     let start_time = Instant::now();

//     let result = func();

//     let duration = start_time.elapsed();
//     sys.refresh_all();
//     let end_memory = sys
//         .process(pid)
//         .expect("Failed to get current process")
//         .memory();

//     println!("Function: {}", func_name);
//     println!("Execution Time: {:.2?}", duration);
//     println!("Memory Usage Before: {} KB", start_memory);
//     println!("Memory Usage After: {} KB", end_memory);
//     println!(
//         "Memory Consumed: {} KB\n",
//         end_memory as i64 - start_memory as i64
//     );

//     result
// }

// fn main() {
//     let url = "https://raw.githubusercontent.com/nogibjj/Mini_PJT_6_Complex-SQL-Query-for-a-MySQL-Database_ISL/main/data_raw/HR_1.csv";
//     let file_path = "HR_1.csv";
//     let timeout = 10;

//     if let Err(e) = performance(|| extract(url, file_path, timeout), "extract") {
//         println!("Error in extract: {}", e);
//     }
//     if let Err(e) = performance(|| load(file_path), "load") {
//         println!("Error in load: {}", e);
//     }
//     if let Err(e) = performance(query_create, "query_create") {
//         println!("Error in query_create: {}", e);
//     }
//     if let Err(e) = performance(query_read, "query_read") {
//         println!("Error in query_read: {}", e);
//     }
//     if let Err(e) = performance(query_update, "query_update") {
//         println!("Error in query_update: {}", e);
//     }
//     if let Err(e) = performance(query_delete, "query_delete") {
//         println!("Error in query_delete: {}", e);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::path::Path;

//     #[test]
//     fn test_extract() {
//         let url = "https://raw.githubusercontent.com/nogibjj/Mini_PJT_6_Complex-SQL-Query-for-a-MySQL-Database_ISL/main/data_raw/HR_1.csv";
//         let file_path = "HR_1.csv";
//         let timeout = 10;

//         let result = extract(url, file_path, timeout);
//         assert_eq!(result, Ok("HR_1.csv".to_string()));
//     }

//     #[test]
//     fn test_load() {
//         let file_path = "HR_1.csv";

//         // Attempt to load data and check if the DB file is created
//         let _ = load(file_path);

//         // Check if the database file was created as the success condition
//         let db_path = Path::new("HR_1.db");
//         assert!(db_path.exists(), "Database file was not created");
//     }

//     #[test]
//     fn test_query_create() {
//         let result = query_create();
//         assert_eq!(result, Ok("Create Success".to_string()));
//     }

//     #[test]
//     fn test_query_read() {
//         let result = query_read();
//         assert_eq!(result, Ok("Read Success".to_string()));
//     }

//     #[test]
//     fn test_query_update() {
//         let result = query_update();
//         assert_eq!(result, Ok("Update Success".to_string()));
//     }

//     #[test]
//     fn test_query_delete() {
//         let result = query_delete();
//         assert_eq!(result, Ok("Delete Success".to_string()));
//     }
// }

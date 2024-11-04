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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_extract() {
        let url = "https://raw.githubusercontent.com/nogibjj/Mini_PJT_6_Complex-SQL-Query-for-a-MySQL-Database_ISL/main/data_raw/HR_1.csv";
        let file_path = "test_HR_1.csv";
        let timeout = 10;

        // Run the extract operation
        let result = extract(url, file_path, timeout);

        // Ensure the file was created
        assert!(result.is_ok(), "Extract failed: {:?}", result);
        assert!(fs::metadata(file_path).is_ok(), "File not created");

        // Clean up test file
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_load() {
        let file_path = "test_HR_1.csv";

        // Create a dummy file for testing
        fs::write(file_path, "dummy content").expect("Failed to write test file");

        // Run the load operation
        let result = load(file_path);

        // Ensure the load was successful
        assert!(result.is_ok(), "Load failed: {:?}", result);

        // Clean up test file
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_create() {
        // Test the create operation
        let result = create_data();
        assert!(result.is_ok(), "Create failed: {:?}", result);
    }

    #[test]
    fn test_read() {
        // Test the read operation
        let result = read_data();
        assert!(result.is_ok(), "Read failed: {:?}", result);
    }

    #[test]
    fn test_update() {
        // Test the update operation
        let result = update_data();
        assert!(result.is_ok(), "Update failed: {:?}", result);
    }

    #[test]
    fn test_delete() {
        // Test the delete operation
        let result = delete_data();
        assert!(result.is_ok(), "Delete failed: {:?}", result);
    }
}

// use clap::{Parser, Subcommand};
// use mini_pjt_7_rust_pilot_isl::{
//     extract, load, query_create as create_data, query_delete as delete_data,
//     query_read as read_data, query_update as update_data,
// };
// use std::time::Instant;
// use sysinfo::{ProcessExt, System, SystemExt};

// #[derive(Parser)]
// #[command(name = "example")]
// #[command(about = "A Rust program for SQLite ETL and query pipeline with memory and runtime tracking", long_about = None)]
// struct Args {
//     #[arg(
//         short,
//         long,
//         default_value = "https://raw.githubusercontent.com/nogibjj/Mini_PJT_6_Complex-SQL-Query-for-a-MySQL-Database_ISL/main/data_raw/HR_1.csv"
//     )]
//     url: String,

//     #[arg(short, long, default_value = "HR_1.csv")]
//     file_path: String,

//     #[command(subcommand)]
//     action: Action,
// }

// #[derive(Subcommand)]
// enum Action {
//     Extract {
//         #[arg(short, long, default_value_t = 10)]
//         timeout: u64,
//     },
//     Load,
//     Create,
//     Read,
//     Update,
//     Delete,
//     All,
// }

// fn track_memory_and_time<F>(operation_name: &str, operation: F) -> Result<(), String>
// where
//     F: FnOnce() -> Result<String, String>,
// {
//     let start_time = Instant::now();
//     let mut system = System::new_all();
//     system.refresh_all();
//     let pid = sysinfo::get_current_pid().expect("Failed to get process ID");
//     let initial_memory = system.process(pid).map_or(0, |process| process.memory());

//     // Execute the operation
//     let result = operation();

//     // Measure final memory usage
//     system.refresh_all();
//     let final_memory = system.process(pid).map_or(0, |process| process.memory());

//     // Calculate elapsed time and memory usage
//     let elapsed_time = start_time.elapsed();
//     let memory_used = final_memory.saturating_sub(initial_memory);

//     // Print metrics for the operation
//     println!("{} completed in: {:.2?}", operation_name, elapsed_time);
//     println!("Memory used during {}: {} KB", operation_name, memory_used);

//     result.map(|_| ())
// }

// fn main() {
//     let args = Args::parse();

//     match args.action {
//         Action::Extract { timeout } => {
//             if let Err(e) =
//                 track_memory_and_time("Extract", || extract(&args.url, &args.file_path, timeout))
//             {
//                 println!("Error in extract: {}", e);
//             }
//         }
//         Action::Load => {
//             if let Err(e) = track_memory_and_time("Load", || load(&args.file_path)) {
//                 println!("Error in load: {}", e);
//             }
//         }
//         Action::Create => {
//             if let Err(e) = track_memory_and_time("Create", create_data) {
//                 println!("Error in create: {}", e);
//             }
//         }
//         Action::Read => {
//             if let Err(e) = track_memory_and_time("Read", read_data) {
//                 println!("Error in read: {}", e);
//             }
//         }
//         Action::Update => {
//             if let Err(e) = track_memory_and_time("Update", update_data) {
//                 println!("Error in update: {}", e);
//             }
//         }
//         Action::Delete => {
//             if let Err(e) = track_memory_and_time("Delete", delete_data) {
//                 println!("Error in delete: {}", e);
//             }
//         }
//         Action::All => {
//             // Perform all actions in sequence
//             if let Err(e) =
//                 track_memory_and_time("Extract", || extract(&args.url, &args.file_path, 10))
//             {
//                 println!("Error in extract: {}", e);
//             }
//             if let Err(e) = track_memory_and_time("Load", || load(&args.file_path)) {
//                 println!("Error in load: {}", e);
//             }
//             if let Err(e) = track_memory_and_time("Create", create_data) {
//                 println!("Error in create: {}", e);
//             }
//             if let Err(e) = track_memory_and_time("Read", read_data) {
//                 println!("Error in read: {}", e);
//             }
//             if let Err(e) = track_memory_and_time("Update", update_data) {
//                 println!("Error in update: {}", e);
//             }
//             if let Err(e) = track_memory_and_time("Delete", delete_data) {
//                 println!("Error in delete: {}", e);
//             }
//         }
//     }
// }

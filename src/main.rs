use std::fs::{create_dir_all, read_dir, File};
use std::io::{Read, Write};
use std::path::Path;
use tokio::time::Instant;
use clap::Parser;
use chrono::{NaiveDate, Duration};
use std::process;
use futures::future;

#[derive(Parser, Debug)]
#[command(name="Log Exporter", about="To export raw logs to S3")]
struct CLI {
    /// Folder name
    folder_names: String,
    /// start date in YYYY-MM-DD format
    start_date: String,
    /// end date in YYYY-MM-DD format
    end_date: String
}

async fn get_date_ranges(start_date: &str, end_date: &str) -> Vec<String> {
    let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d").unwrap();

    let mut current_date = start_date;
    let mut date_list = Vec::new();

    while current_date <= end_date {
        date_list.push(current_date.to_string());
        current_date = current_date + Duration::days(1);
    }
    date_list
}

async fn get_full_path(folder_name: &str, date: &str) -> String {
    let path_vec: Vec<&str> = date.split("-").collect();
    let path = path_vec.join("/");
    let full_path = format!("{}/{}/{}", "logs", folder_name, path);
    full_path
}

async fn export_folder_files(folder_path: &str) -> Result<String, String> {
    let output_folder = format!("logs/exported/{}", folder_path);
    if !Path::new(&output_folder).exists() {
        create_dir_all(&output_folder).unwrap();
        println!("Created folder: {}", &output_folder);
    }
    let output_file = format!("logs/exported/{}/{}", folder_path, "raw_log.txt");
    let mut output = File::create(output_file).expect("Could not create output file");

    let entries = read_dir(folder_path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let mut input_file = File::open(&path).unwrap();
            let mut buffer = [0; 12288];

            loop {
                let bytes_read = input_file.read(&mut buffer).unwrap();
                if bytes_read == 0 {
                    break;
                }
                output.write_all(&buffer[..bytes_read]).unwrap();
            }
        }
    }
    Ok(format!("Exported to {}", "logs/exported").to_string())
}

async fn check_export_files(folder_name: &str, start_date: &str, end_date: &str) {
    let all_possible_folders = get_date_ranges(start_date, end_date).await;

    for folder in all_possible_folders.iter() {
        let path = get_full_path(folder_name, folder).await;
        if Path::new(&path).exists() {
            let _ = export_folder_files(&path).await;
        } else {
            println!("Path does not exist: {}", path);
        }
    }
}

async fn validate_supplied_date(start_date: &str, end_date: &str) -> Result<String, String> {
    match NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
        Ok(start_date) => match NaiveDate::parse_from_str(end_date, "%Y-%m-%d") {
            Ok(end_date) => {
                if end_date < start_date {
                    Err("End date is smaller than start date".to_string())
                } else {
                    Ok("Valid date range".to_string())
                }
            }
            Err(_) => Err("Invalid end date".to_string()),
        },
        Err(_) => Err("Invalid start date".to_string()),
    }
}

#[tokio::main]
async fn main() {
    let process_id = process::id();
    println!("Starting!\n--------- PID: {process_id} --------------\n");

    let args = CLI::parse();

    let folder_names = args.folder_names;
    let start_date = args.start_date;
    let end_date = args.end_date;

    let start = Instant::now();

    match validate_supplied_date(&start_date, &end_date).await {
        Ok(_) => {
            let folders: Vec<String> = folder_names.split(',').map(|s| s.trim().to_string()).collect();
            let mut tasks = Vec::new();

            for folder in folders {
                let start_date = start_date.clone();
                let end_date = end_date.clone();
                let task = tokio::spawn(async move {
                    check_export_files(&folder, &start_date, &end_date).await;
                });
                tasks.push(task);
            }

            future::join_all(tasks).await;
        }
        Err(e) => println!("Error: {}", e),
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    println!("\n------------------------\nFinished!");
}

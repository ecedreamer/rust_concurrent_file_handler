use std::path::Path;
use clap::Parser;
use chrono::{NaiveDate, Duration};

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



fn get_date_ranges(start_date: &str, end_date: &str) -> Vec<String> {
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


fn get_full_path(folder_name: &str, date: &str) -> String {
    let path_vec: Vec<&str> = date.split("-").collect();
    let path = path_vec.join("/");
    let full_path = format!("{}/{}/{}", "logs", folder_name, path);
    full_path
}


fn export_files(folder_name: &str, start_date: &str, end_date: &str) {
    let all_possible_folders = get_date_ranges(start_date, end_date);
    println!("{:?}", all_possible_folders);
    for folder in all_possible_folders.iter() {
        let path = get_full_path(folder_name, folder);
        if Path::new(&path).exists() {
            println!("Path exists: {}", path);
        } else {
            println!("Path does not exists: {}", path);
        }
    }

}


fn validate_supplied_date(start_date: &str, end_date: &str) -> Result<String, String> {
    match NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
        Ok(start_date) => {
            match NaiveDate::parse_from_str(end_date, "%Y-%m-%d") {
                Ok(end_date) => {
                    if end_date < start_date {
                        Err("End date is smaller than start date".to_string())
                    } else {
                        Ok("Valid date range".to_string())
                    }
                },
                Err(e) => Err("Invalid end date".to_string()),
            }
        },
        Err(e) => Err("Invalid start date".to_string()),
    }
}

fn main() {
    println!("Starting!\n-----------------------\n");

    let args = CLI::parse();

    let folder_names = args.folder_names;

    let start_date = args.start_date;
    let end_date = args.end_date;

    match validate_supplied_date(&start_date, &end_date) {
        Ok(msg) => {
            let folders: Vec<String> = folder_names.split(",").map(|s| s.trim().to_string()).collect();
            for folder in folders.iter() {
                export_files(&folder, &start_date, &end_date);
            }
        },
        Err(e) => println!("Error: {}", e)
    }



    println!("\n------------------------\nFinished!");
}

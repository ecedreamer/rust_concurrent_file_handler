use clap::Parser;



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



fn main() {
    println!("Starting!\n-----------------------\n");

    let args = CLI::parse();

    let folder_names = args.folder_names;
    let folders: Vec<String> = folder_names.split(",").map(|s| s.trim().to_string()).collect();
    let start_date = args.start_date;
    let end_date = args.end_date;


    for folder in folders.iter() {
        println!("Exporting logs from folder: {}", folder);
        println!("Start date: {:?}", start_date);
        println!("End date: {:?}", end_date);
    }


    println!("\n------------------------\nFinished!");
}

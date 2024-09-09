from pathlib import Path
import time
from datetime import datetime



BASE_FOLDERS = ("folder_1", "folder_2", "folder_3")
BASE_YEARS = ("2024", )
BASE_MONTHS = ("09", )
BASE_DAYS = [str(DAY).zfill(2) for DAY in range(1, 33)]


def generate_log_files():
    for folder in BASE_FOLDERS:
        base_folder_dir = Path(folder)
        base_folder_dir.mkdir(parents=False, exist_ok=True)
        for year in BASE_YEARS:
            base_year_dir = Path(base_folder_dir / year)
            base_year_dir.mkdir(parents=False, exist_ok=True)
            for month in BASE_MONTHS:
                base_month_dir = Path(base_year_dir / month)
                base_month_dir.mkdir(parents=False, exist_ok=True)
                for day in BASE_DAYS:
                    base_day_dir = Path(base_month_dir / day)
                    base_day_dir.mkdir(parents=False, exist_ok=True)
                    date_str = f"{year}-{month}-{day} 09:00:00"
                    date_format = '%Y-%m-%d %H:%M:%S'
                    try:
                        date_time_obj = datetime.strptime(date_str, date_format)
                    except:
                        continue
                    timestamp = int(date_time_obj.timestamp())
                    for _ in range(20):
                        file_name = f"{folder}/{year}/{month}/{day}/{timestamp}_file"
                        with open(file_name, "w") as file:
                            for i in range(100000):
                                content = f"File name is {file_name} and the start timestamp is {timestamp} and i is {i}\n"
                                file.write(content)
                        timestamp += 600
        print(f"Folder: {folder} is ready")






def main() -> None:
    print("Starting log generator")

    generate_log_files()



if __name__ == "__main__":
    main()

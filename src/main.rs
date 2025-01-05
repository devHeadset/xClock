use chrono::Local;

fn main() {
    // Get the current local time
    let current_time = Local::now();
    
    // Format the time with hours, minutes, and seconds
    let formatted_time = current_time.format("%H:%M:%S");
    
    println!("{}", formatted_time);
}


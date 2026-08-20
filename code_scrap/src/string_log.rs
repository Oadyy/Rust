use std::collections::HashMap;

struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
    /// use all String not &str because it want to owner and feild not unknow expect size.
}

fn parse_log_line(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return None;
    }

    let timestamp = parts[0].to_string();
    let level = parts[1].to_string();
    let message = parts[2].to_string();

    Some(LogEntry {
        timestamp,
        level,
        message,
    })
}

fn analyze_logs(logs: &Vec<String>) -> HashMap<String, usize> {
    logs.inter() // use inter() it will return &T in this situation is &str
        .filter_map(|line| parse_log_line(line)) // filter_map() is ??
        .fold(HashMap::new(), |mut acc, entry| { // fold() is ??
            *acc.entry(entry.level).or_insert(0) += 1;
            acc
        })
}

fn demo() {
    let logs = vec![
        "2024-06-01 12:00:00 INFO Application started".to_string(),
        "2024-06-01 12:01:00 WARN Low disk space".to_string(),
        "2024-06-01 12:02:00 ERROR Failed to connect to database".to_string(),
        "2024-06-01 12:03:00 INFO User logged in".to_string(),
        "2024-06-01 12:04:00 WARN High memory usage".to_string(),
        "2024-06-01 12:05:00 ERROR Timeout while processing request".to_string(),
    ];

    let log_summary = analyze_logs(&logs);

    println!("Log Summary:");
    for (level, count) in log_summary {
        println!("{}: {}", level, count);
    }

    let search_term = "User";
    let matching_logs: Vec<&String> = logs.
    .inter()
    .filter(|line| line.contains(search_term))
    .collect();

    for log in matching_logs {
        println!("{}", log);
    }
}
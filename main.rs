
use std::thread;
use std::time::Duration;

fn main() {
    // Example thread
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("[thread] count = {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Main thread
    for i in 1..=3 {
        println!("[main] count = {}", i);
        thread::sleep(Duration::from_millis(80));
    }

    handle.join().expect("Thread panicked");
    println!("All done");

    // ==========================
    // TODO 1 Solution
    // ==========================

    let mut handles = Vec::new();

    let ranges = vec![
        (1, 250),
        (251, 500),
        (501, 750),
        (751, 1000),
    ];

    for (start, end) in ranges {
        let handle = thread::spawn(move || {
            let mut sum = 0;

            for i in start..=end {
                sum += i;
            }

            println!("Sum from {} to {} = {}", start, end, sum);

            sum
        });

        handles.push(handle);
    }

    let mut total = 0;

    for handle in handles {
        total += handle.join().expect("Thread panicked");
    }

    println!("Total Sum = {}", total);

    //B

      // ==========================
    // Part 1: Original Program
    // ==========================
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    for _ in 0..8 {
        let c = Arc::clone(&counter);

        handles.push(thread::spawn(move || {
            for _ in 0..1_000 {
                let mut num = c.lock().unwrap();
                *num += 1;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());
    // Should always print 8000

    // ==========================
    // TODO 2 Solution
    // Each thread accumulates locally
    // and locks the mutex only once.
    // ==========================

    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    let start = Instant::now();

    for _ in 0..8 {
        let c = Arc::clone(&counter);

        handles.push(thread::spawn(move || {
            let mut local_sum = 0u64;

            for _ in 0..1_000 {
                local_sum += 1;
            }

            let mut num = c.lock().unwrap();
            *num += local_sum;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let duration = start.elapsed();

    println!("Optimized counter: {}", *counter.lock().unwrap());
    println!("Elapsed time: {:?}", duration);

    //C


    let (tx, rx) = mpsc::channel();
    let dataset: Vec<Vec<u64>> = (0..4)
        .map(|i| (i*250+1..=(i+1)*250).collect())
        .collect();

    for (id, chunk) in dataset.iter().enumerate() {
        let tx_clone = tx.clone();
        // Need to clone the chunk data since we're moving it into the thread
        let chunk_clone = chunk.clone();
        thread::spawn(move || worker(id, chunk_clone, tx_clone));
    }

    drop(tx); // close the original sender so rx knows when all are done

    // Updated receiver to handle both Sum and Error variants
    let mut total: u64 = 0;
    let mut errors: Vec<String> = Vec::new();
    
    for result in rx.iter() {
        match result {
            WorkResult::Sum(s) => total += s,
            WorkResult::Error(e) => {
                println!("Error received: {}", e);
                errors.push(e);
            }
        }
    }
    
    println!("Grand total (successful sums only): {} (expected 125250)", total);
    
    if !errors.is_empty() {
        println!("Encountered {} error(s):", errors.len());
        for error in errors {
            println!("  - {}", error);
        }
    }


    //D

     fn main() ->io::Result<()> {
    let path = "output.log";
    let entries = vec![
        "INFO Server started",
        "WARN High memory usage",
        "ERROR Disk full",
        "INFO Backup complete"
    ];
    write_log(path, &entries)?;

    let n = count_lines(path)?;
    println!("Wrote {} lines to {}", n, path);

    // Read back and filter
    let content = fs::read_to_string(path)?;
    let errors: Vec<&str> = content.lines()
        .filter(|l| l.starts_with("ERROR"))
        .collect();
    println!("Error lines: {:?}", errors);

    fs::remove_file(path)?;

    // Test the recursive file listing function
    println!("\n=== Recursive .rs file search ===");
    match list_rs_files(Path::new(".")) {
        Ok(files) => {
            println!("Found {} .rs files:", files.len());
            for file in files {
                println!("  {}", file);
            }
        }
        Err(e) => println!("Error searching for .rs files: {}", e),
    }
    
    // You can also test with a specific directory
    // match list_rs_files_with_path("src") {
    //     Ok(files) => {
    //         println!("Found {} .rs files in src/:", files.len());
    //         for file in files {
    //             println!("  {}", file);
    //         }
    //     }
    //     Err(e) => println!("Error: {}", e),
    // }

    Ok(())
     }
    
}


//Exercise B

use std::sync::{Arc, Mutex};
use std::time::Instant;


//Exercise_C


use std::sync::mpsc;


#[derive(Debug)]
enum WorkResult {
    Sum(u64),
    Error(String),
}

fn worker(id: usize, data: Vec<u64>, tx: mpsc::Sender<WorkResult>) {
    let sum: u64 = data.iter().sum();
    println!("Worker {} computed sum = {}", id, sum);
    
    // TODO 3: Add an Error variant — if a chunk's sum > 30000,
    //    send WorkResult::Error instead. Handle it in the receiver.
    if sum > 30000 {
        tx.send(WorkResult::Error(format!("Worker {}: sum {} exceeds 30000", id, sum))).unwrap();
    } else {
        tx.send(WorkResult::Sum(sum)).unwrap();
    }
}


//Exercise_D

use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn write_log(path: &str, entries: &[&str]) -> io::Result<()> {
    let mut file = File::create(path)?;
    for entry in entries {
        writeln!(file, "{}", entry)?;
    }
    Ok(())
}

fn count_lines(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().count())
}

// TODO 4: Write a function that recursively lists all .rs files under a given directory using std::fs::read_dir.
fn list_rs_files(dir: &Path) -> io::Result<Vec<String>> {
    let mut rs_files = Vec::new();
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Recursively search subdirectories
                let mut sub_files = list_rs_files(&path)?;
                rs_files.append(&mut sub_files);
            } else if let Some(extension) = path.extension() {
                if extension == "rs" {
                    if let Some(path_str) = path.to_str() {
                        rs_files.push(path_str.to_string());
                    }
                }
            }
        }
    }
    
    Ok(rs_files)
}

// Alternative version with a closure for more flexibility
fn list_rs_files_with_path<P: AsRef<Path>>(dir: P) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    let dir = dir.as_ref();
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                files.append(&mut list_rs_files_with_path(&path)?);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
        }
    }
    
    Ok(files)
}

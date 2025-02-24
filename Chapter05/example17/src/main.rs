use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

fn backup_data(data: &str,filename:&str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn main() {
    let financial_data = vec![
        "Data1",
        "Data2",
        "Data3",
    ];

    let data_mutex = Arc::new(Mutex::new(financial_data));
    let mut thread_handles = vec![];

    for (index,data) in data_mutex.lock().unwrap().iter_mut().enumerate(){
        let filename = format!("financial_data_{}.txt",index);
        let data = data.to_string();
        let handle = thread::spawn(move || {
            match backup_data(&data, &filename) {
                Ok(_) => println!("Backup successful: {}",filename),
                Err(err) => eprintln!("Error backing up {}:{:?}",filename,err),
            }
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}

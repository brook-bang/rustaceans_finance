use std::{sync::{Arc, Mutex}, thread};


#[allow(dead_code)]
#[derive(Debug)]
struct FinancialData {
    value: f64,
}


fn main() {
    let shared_data_pool: Arc<Mutex<Vec<Box<FinancialData>>>> = Arc::new(Mutex::new(Vec::new()));

    let num_writers = 4;
    let mut writer_handles = vec![];

    for i in 0..num_writers {
        let shared_data_pool = Arc::clone(&shared_data_pool);
        let handle = thread::spawn(move ||{
            let new_data = FinancialData {
                value: i as f64 * 100.0,
            };

            let mut data_pool = shared_data_pool.lock().unwrap();
            data_pool.push(Box::new(new_data));
        });
        writer_handles.push(handle);
    }

    let num_readers = 2;
    let mut reader_handles = vec![];

    for _ in 0..num_readers {
        let shared_data_pool = Arc::clone(&shared_data_pool);

        let handle = thread::spawn(move || {
            let data_pool = shared_data_pool.lock().unwrap();
            for data in &*data_pool {
                println!("Reader thread - Data: {:?}",data);
            }
        });
        reader_handles.push(handle);
    }

    for handle in writer_handles {
        handle.join().unwrap();
    }

    for handle in reader_handles {
        handle.join().unwrap();
    }

    

}

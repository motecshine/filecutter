use dotenv::dotenv;
use std::env;
use threadpool::ThreadPool;

pub mod dir_scanner;
pub mod file_cutter;

fn main() {
    dotenv().ok();
    let thread_pool_num = env::var("THREAD_POOL").unwrap().parse::<usize>().unwrap();
    let thread_pool = ThreadPool::with_name("file_cutter_worker".to_string(), thread_pool_num);
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    let mut tx_clone = tx.clone();
    thread_pool.execute(move || dir_scanner::new_dir_scanner(&mut tx_clone).unwrap().run());
    let file_cutter = file_cutter::new_cutter().unwrap();
    loop {
        if thread_pool.active_count() >= thread_pool_num {
            thread_pool.join();
        }
        match rx.recv() {
            Ok(path) => {
                let mut fc_clone = file_cutter.clone();
                println!("当前活跃worker:{:?}", thread_pool.active_count());
                thread_pool.execute(move || match fc_clone.chunk(path) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("recv err{:?}.", err);
                    }
                })
            }
            Err(err) => {
                println!("recv err{:?}", err);
            }
        }
    }
}

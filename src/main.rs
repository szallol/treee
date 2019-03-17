extern crate tokio;
extern crate futures;

use tokio::prelude::*;
use tokio::fs;
use futures::{Future, Stream};

fn visit_dirs_par(dir: String){
    let future = fs::read_dir(dir)
        .flatten_stream()
        .for_each( move |entry| {
            let std_entry = entry.into_std();
            let is_dir = std_entry.metadata().unwrap().is_dir();
            if is_dir {
                let entry_path = std_entry.path().into_os_string().into_string().unwrap();
                println!("{:?}", entry_path);
                visit_dirs_par(entry_path);
            }

            future::ok(())
        })
        .map_err(|e| eprintln!("Error reading directory: {}", e));

    tokio::spawn(future);
}

fn main() {
    let visit_future = future::poll_fn( move || {
        visit_dirs_par(String::from("c:/"));
        Ok(Async::Ready(()))
    });

    tokio::run(visit_future);
}

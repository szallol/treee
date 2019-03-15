extern crate tokio;
extern crate futures;

use tokio::prelude::*;
use tokio::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::io::Error;
use futures::{Future, Stream};
use futures::future::poll_fn;
use std::io;

fn print_dir_entry(entry : &DirEntry){
    println!("{:?}", entry);
}

fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn visit_dirs_par(dir: String) {
    let future = fs::read_dir(dir)
        .flatten_stream()
        .for_each(|entry| {
            let entry_path = entry.path();
            poll_fn(move || entry.poll_file_type()).map( move |file_type| {
                if file_type.is_dir() {
                    visit_dirs_par(entry_path.into_os_string().into_string().unwrap());
                }
                else {
                    println!("{:?}", entry_path);
                }
            })
        })
        .map_err(|e| eprintln!("Error reading directory: {}", e));

    tokio::spawn(future);
}

fn main() {
    let visit_future = future::poll_fn( || {
        visit_dirs_par(String::from("c:/work/"));
        Ok(Async::Ready(()))
    });

    tokio::run(visit_future);
//    visit_dirs(Path::new("\\Windows"), &print_dir_entry);
}

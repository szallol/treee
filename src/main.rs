extern crate rayon;

use rayon::prelude::*;

use std::fs;
use std::path::Path;
//use std::thread;
//use std::sync::mpsc::{Sender, Receiver};
//use std::sync::mpsc;
//use std::option::Option;

fn visit_dirs(dir: &Path) {
    if dir.is_dir() {
        let dirs: Vec<String> = fs::read_dir(dir).unwrap()
            .filter_map(|r| {
                let path = r.unwrap().path();
                if path.is_dir() {
                    let path = path.display().to_string();
//                    println!("{:?}", path);
                    Some(path)
                }
                else { None }
            })
            .collect();

//        dirs.into_iter()
//            .for_each(|entry: String| {
//            });

        dirs.par_iter()
            .for_each(|entry| {
                visit_dirs(Path::new(entry));
            });
    }
}

fn main() {
//    let (sender, receiver):(Sender<Vec<String>>, Receiver<Vec<String>>) = mpsc::channel();

//    thread::spawn(move || {
//
//    });

    visit_dirs(Path::new("c:/"));


//    tokio::run(futures::lazy(move || {
//        tokio::spawn(
//            rx.for_each(move |value : String| {
////                println!("{}", value);
//                io::stdout().write(value.as_bytes()).unwrap();
//                io::stdout().write(b"\n").unwrap();
//                Ok(())
//            })
//        );
//
//        let visit_future = future::poll_fn( move || {
//            visit_dirs_par(String::from("c:/"), tx.clone());
//            Ok(Async::Ready(()))
//        });
//        tokio::spawn(visit_future);
//
//        Ok(())
//    })
//    );

}

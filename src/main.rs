extern crate tokio;
extern crate futures;

use tokio::prelude::*;
use tokio::fs;
use futures::sync::mpsc;
use futures::{Future, Stream};

fn visit_dirs_par(dir: String, tx: mpsc::Sender<String>) {
    let future = fs::read_dir(dir)
        .flatten_stream()
        .for_each( move |entry| {
            let std_entry = entry.into_std();
            let is_dir = std_entry.metadata().unwrap().is_dir();
            if is_dir {
                let entry_path = std_entry.path().into_os_string().into_string().unwrap();
                visit_dirs_par(entry_path.clone(), tx.clone());
                tokio::spawn(tx.clone().send(entry_path)
                    .map(|_| ())
                    .map_err(|_| ()));
            }

            future::ok(())
        })
        .map_err(|_| ());

   tokio::spawn(future);
}

fn main() {

    let (tx, rx) = mpsc::channel(1);

    tokio::run(futures::lazy(move || {

        tokio::spawn(
            rx.for_each(move |value| {
                println!("{}", value);
                Ok(())
            })
        );

        let visit_future = future::poll_fn( move || {
            visit_dirs_par(String::from("c:/"), tx.clone());
            Ok(Async::Ready(()))
        });
        tokio::spawn(visit_future);

        Ok(())
    })
    );

}

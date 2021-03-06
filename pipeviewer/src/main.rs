use crossbeam::channel::{bounded, unbounded};
use pipeviewer::{args::Args, read, stats, write};
use std::io::Result as IoResult;
use std::thread;

fn main() -> IoResult<()> {
    let Args {
        infile,
        outfile,
        silent,
    } = Args::parse();

    let (stats_tx, stats_rx) = unbounded();
    let (write_tx, write_rx) = bounded(1024);

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx, write_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    read_handle.join().unwrap()?;
    stats_handle.join().unwrap()?;
    write_handle.join().unwrap()?;

    Ok(())
}

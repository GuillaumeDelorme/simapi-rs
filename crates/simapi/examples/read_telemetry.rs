use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

fn main() -> simapi::Result<()> {
    let running = Arc::new(AtomicBool::new(true));

    {
        let running = Arc::clone(&running);

        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
        })
        .expect("failed to install Ctrl-C handler");
    }

    let mut client = simapi::Client::open()?;

    println!("Connected to simapi.");
    println!("Press Ctrl-C to stop.");
    println!();

    while running.load(Ordering::SeqCst) {
        let snapshot = client.snapshot()?;

        println!(
            "sim_on={:<5} status={:?} velocity={:>4} rpms={:>5} gear={:<2} gas={:.2} brake={:.2} clutch={:.2} fuel={:.1}/{:.1} car='{}' track='{}'",
            snapshot.sim_on,
            snapshot.status,
            snapshot.velocity,
            snapshot.rpms,
            snapshot.gearc,
            snapshot.gas,
            snapshot.brake,
            snapshot.clutch,
            snapshot.fuel,
            snapshot.fuel_capacity,
            snapshot.car,
            snapshot.track,
        );

        thread::sleep(Duration::from_millis(100));
    }

    println!("Stopped.");

    Ok(())
}

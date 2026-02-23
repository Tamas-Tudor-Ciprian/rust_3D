use std::time::{Instant,Duration};
use std::sync::atomic::{AtomicU64,Ordering};
use std::sync::Mutex;

static DELTA_TIME_NS: AtomicU64 = AtomicU64::new(0);
static LAST_INSTANT: Mutex<Option<Instant>> = Mutex::new(None);



pub fn get()-> f64 {
	Duration::from_nanos(DELTA_TIME_NS.load(Ordering::Relaxed)).as_secs_f64()
	}


pub fn store() {
	let now = Instant::now();
	let mut last = LAST_INSTANT.lock().unwrap();
	let dt = match *last {
		Some(prev) => now.duration_since(prev),
		None => Duration::ZERO,
	};
	*last = Some(now);

	DELTA_TIME_NS.store(dt.as_nanos() as u64, Ordering::Relaxed);

	}

use crossterm::{
		cursor,
		terminal::{self,ClearType,enable_raw_mode, disable_raw_mode},
		event::{self, Event, KeyCode},
		ExecutableCommand,
		style::*,
		};



use std::io::{stdout, Write,Stdout};
use std::thread::sleep;
use std::time::{Instant,Duration};
use std::sync::atomic::{AtomicU64,Ordering};


use rand::Rng;


static DELTA_TIME_NS: AtomicU64 = AtomicU64::new(0);

const SCREEN_MEASURES: (i32,i32) = (120,30);




fn get_delta_time() ->Duration {
	Duration::from_nanos(DELTA_TIME_NS.load(Ordering::Relaxed))

	}


fn main(){

	let _ = enable_raw_mode();

	let mut last = Instant::now();


	let mut stdout = stdout();

	terminal::enable_raw_mode().unwrap();
	stdout.execute(terminal::EnterAlternateScreen).unwrap();
	stdout.execute(terminal::Clear(ClearType::All)).unwrap();


	//this is not really neccessary
	
	sleep(Duration::from_millis(50));


	loop{

	//this is the logic so that you can have
	// that sweet delta time available
	let now = Instant::now();
	let dt = now.duration_since(last);
	last = now;
	
	DELTA_TIME_NS.store(dt.as_nanos() as u64, Ordering::Relaxed);


	if event::poll(Duration::from_millis(0)).unwrap_or(false) {
		if let Ok(Event::Key(key)) = event::read(){
			match key.code {
				KeyCode::Char('q') => break,
				KeyCode::Left =>{},
				KeyCode::Right => {},
				KeyCode::Up => {},
				KeyCode::Down => {},
				_ => {},
			


			}
		}
	}


	}

	

	//stdout.execute(cursor::Show).unwrap();
	stdout.execute(terminal::LeaveAlternateScreen).unwrap();
	terminal::disable_raw_mode().unwrap();
	
	let _ = disable_raw_mode();




}

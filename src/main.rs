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

const SCREEN_MEASURES: (i32,i32) = (156,50);




fn get_delta_time() ->Duration {
	Duration::from_nanos(DELTA_TIME_NS.load(Ordering::Relaxed))

	}


struct Vector {

	x:f64,
	y:f64,

	}


struct Player {
	position : Vector,
	angle : f64,
	}

struct Ray {
	origin : Vector,
	direction : Vector,
	angle : f64,
	}

struct Line{
	a: Vector,
	b: Vector
	}


fn ray_line_delta(r : &Ray, l :&Line) -> f64{

	//first we construct the points in the ray and line equations:

	let o = &r.origin;
	let b = &l.b;
	let a = &l.a;


	let e = Vector {x:(b.x - a.x),y: (b.y - b.y)};
	let f = Vector {x:(o.x - a.x),y:(o.y - a.y)};
	let d = Vector {x:r.direction.x , y:r.direction.y};


	let u = (f.x*d.y - f.y*d.x) / (d.x*e.y - d.x*e.y);


	u	

}

fn make_frame(out: &mut Stdout){

	
	let edge_char : String = "🧱".to_string();


        //this are the horizontal edges
	out.execute(cursor::MoveTo(0,1)).unwrap();

	for _ in 0..(SCREEN_MEASURES.0/2){
		write!(out,"{}",edge_char).unwrap();
	}


	out.execute(cursor::MoveTo(0,SCREEN_MEASURES.1 as u16)).unwrap();
	
	for _ in 0..(SCREEN_MEASURES.0/2){
		write!(out,"{}",edge_char).unwrap();
	}

	//now there are the vertical borders
	for i in 0..(SCREEN_MEASURES.1 ){
		out.execute(cursor::MoveTo(0,i as u16)).unwrap();
		write!(out,"{}",edge_char).unwrap();
		out.execute(cursor::MoveTo((SCREEN_MEASURES.0 - 2) as u16, i as u16)).unwrap();
		write!(out,"{}", edge_char).unwrap();
	}	


}

fn display_minimap(out: &mut Stdout) {



	let minimap = vec![
			"#####################",
			"#             ##### #",
			"## ##########       #",
			"##              #####",
			"## #####   ##########",
			"## # # #           ##",
			"##     ########### ##",
			"##            W#   ##",
			"#####################",
			];
	let mut i = 0;
	for row in minimap{

		out.execute(cursor::MoveTo((SCREEN_MEASURES.0 - 50).try_into().unwrap(),(i + 3).try_into().unwrap()));
		write!(out,"{}",row);
		out.flush();
		i += 1;
	}
}





fn main(){

	let _ = enable_raw_mode();

	let mut last = Instant::now();


	let mut stdout = stdout();

	terminal::enable_raw_mode().unwrap();
	stdout.execute(terminal::EnterAlternateScreen).unwrap();
	stdout.execute(terminal::Clear(ClearType::All)).unwrap();



	make_frame(&mut stdout);


	//this is not really neccessary
	
	sleep(Duration::from_millis(50));


	display_minimap(&mut stdout);


	// this be the main game loop
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

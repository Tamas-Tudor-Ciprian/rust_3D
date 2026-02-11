mod vectors;

use vectors::*;


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

use std::f64::consts::PI;

static DELTA_TIME_NS: AtomicU64 = AtomicU64::new(0);

const SCREEN_MEASURES: (i32,i32) = (156,50);
const FOV: f64 = PI/2.0;



fn get_delta_time() ->f64 {
	Duration::from_nanos(DELTA_TIME_NS.load(Ordering::Relaxed)).as_secs_f64()

	}



struct Player {
	position : Vec2,
	angle : f64,
	speed : f64,
	}


impl Player{
	fn rotate_left(&mut self){
		self.angle += self.speed * get_delta_time();
		}
	fn rotate_right(&mut self){
		self.angle -= self.speed * get_delta_time();
		}

	fn move_right(&mut self){
		self.position.x += self.speed * get_delta_time();
		}
	fn move_left(&mut self){
		self.position.x -= self.speed * get_delta_time();
			}
	fn move_up(&mut self){
		self.position.y += self.speed* get_delta_time();
		}
	fn move_down(&mut self){
		self.position.y -= self.speed* get_delta_time();
		}
	



	}


impl Default for Player{
	fn default() -> Self{
		Self{position : Vec2{x : 0.0, y: 0.0},
		angle : 0.0,
		speed : 0.0,
		}}
	}

struct Ray {
	o : Vec2,
	d : Vec2,
	angle : f64,
	}

struct Line{
	a: Vec2,
	b: Vec2,
	}


fn ray_line_delta(r : &Ray, l :&Line) -> (f64,f64){
	
	let e = l.b - l.a;

	let t = ((l.a - r.o) * e) / (r.d * e);
	let u = ((l.a - r.o) * r.d) / (r.d * e);


	(t,u)

}



fn make_frame(out: &mut Stdout){

	
	let edge_char : String = "🧱".to_string();


        //this are the horizontal edges
	out.execute(cursor::MoveTo(0,0)).unwrap();

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


//this function loads a line into the buffer
fn load_line(buffer :&mut Vec<Vec<u8>>,x : usize, len : usize){

	let buffer_collumn_size = buffer.len();
	let start_point : usize = buffer_collumn_size/2 - len as usize/2;

	for i in 0..len{
		
		buffer[i + start_point][x] = 1;
		}


	}



//this function actually displays the buffer only changing on the screen the differences between the buffer and the prebuffer
fn display_buffer(out: &mut Stdout,buffer: &mut Vec<Vec<u8>>, pre_screen: &mut Vec<Vec<u8>>){

	for (i,line) in buffer.iter().enumerate(){
		for (j,val) in line.iter().enumerate(){
			if *val != pre_screen[i][j]{
				pre_screen[i][j] = *val;
				out.execute(cursor::MoveTo(i as u16,j as u16));
				if *val == 1{write!(out,"█");}
				else{write!(out," ");}
				}

			}
	
		}

}

fn render_fov(out: &mut Stdout,player : Player, lines : Vec<Line>){




	}


fn main(){


	let mut player = Player::default();
	let line = Line{
		a: Vec2{x:-5.0,y:10.0},
		b: Vec2{x:5.0,y:10.0},
		};


		

	//this will be the buffer you actually make logic changes to
	let buffer: [[u8;SCREEN_MEASURES.0 as usize];SCREEN_MEASURES.1 as usize] = [[0;SCREEN_MEASURES.0 as usize];SCREEN_MEASURES.1 as usize];
        //this is the buffer that only gets changed in the differences between it and the buffer to minimize
	//write operations on the console
	let pre_buffer: [[u8;SCREEN_MEASURES.0 as usize];SCREEN_MEASURES.1 as usize] = [[0;SCREEN_MEASURES.0 as usize];SCREEN_MEASURES.1 as usize];

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
				KeyCode::Char('q') => player.rotate_left(),
				KeyCode::Char('e') => player.rotate_right(),
				KeyCode::Left =>player.move_left(),
				KeyCode::Right => player.move_right(),
				KeyCode::Up => player.move_left(),
				KeyCode::Down => player.move_right(),
				KeyCode::Esc => break,
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

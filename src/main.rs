mod vectors;

use vectors::*;


use crossterm::{
		cursor,
		terminal::{self,ClearType,enable_raw_mode, disable_raw_mode},
		event::{self, Event, KeyCode, KeyEventKind},
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
const RENDER_DISTANCE : f64 = 25.0;
const PLAYER_SPEED : f64 = 200.0;


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
		self.position.x += (self.angle + PI/2.0).cos() * self.speed * get_delta_time();
		self.position.y += (self.angle + PI/2.0).sin() * self.speed * get_delta_time();
	}
	fn move_left(&mut self){
		self.position.x += (self.angle - PI/2.0).cos() * self.speed * get_delta_time();
		self.position.y += (self.angle - PI/2.0).sin() * self.speed * get_delta_time();
			}
	fn move_up(&mut self){
		self.position.x += self.angle.cos() * self.speed * get_delta_time();
		self.position.y += self.angle.sin() * self.speed * get_delta_time();
		}
	fn move_down(&mut self){
	
		self.position.x -= self.angle.cos() * self.speed * get_delta_time();
		self.position.y -= self.angle.sin() * self.speed * get_delta_time(); }	



	}


impl Default for Player{
	fn default() -> Self{
		Self{position : Vec2{x : 0.0, y: 0.0},
		angle : 0.0,
		speed : PLAYER_SPEED,
		}}
	}

struct Ray {
	o : Vec2,
	d : Vec2,
	}

impl Ray{
	fn from_angle(angle: f64) -> Vec2{
		Vec2{x:angle.cos(),y:angle.sin()}
	}


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
			"#  ##########       #",
			"#               #####",
			"## #####   ##########",
			"#  # # #            #",
			"##     ############ #",
			"#             W#    #",
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
fn load_line(buffer :&mut Vec<Vec<u8>>,x : usize, len : usize, val : &u8){

	let row_count = buffer.len(); // number of rows (SCREEN_MEASURES.1)
	let len = len.min(row_count);  // clamp so we don't exceed screen height
	if len == 0 { return; }
	let start_point : usize = row_count/2 - len/2;

	for i in 0..len{
		
		buffer[i + start_point][x] = *val; // write vertically: row varies, column is fixed
		}


	}



//this function actually displays the buffer only changing on the screen the differences between the buffer and the prebuffer
fn display_buffer(out: &mut Stdout,buffer: &mut Vec<Vec<u8>>, pre_screen: &mut Vec<Vec<u8>>){

	for (i,line) in buffer.iter().enumerate(){
		for (j,val) in line.iter().enumerate(){
			if *val != pre_screen[i][j]{
				pre_screen[i][j] = *val;
				out.execute(cursor::MoveTo(j as u16,i as u16));
		
				match *val {
					1 => write!(out,"█").unwrap(),
					2 => write!(out,"▓").unwrap(),
					3 => write!(out,"▒").unwrap(),
					4 => write!(out,"░").unwrap(),
					_ => write!(out," ").unwrap(),

				}


			}
	
		}

}
}

fn render_fov(buffer: &mut Vec<Vec<u8>>,player : &Player, lines : &Vec<Line>){

	let mut rays: Vec<Ray> = Vec::new();


	let initial_angle = player.angle - FOV/2.0;
	let angle_increment = FOV/SCREEN_MEASURES.0 as f64;

	for i in 0..SCREEN_MEASURES.0 {
		
		rays.push(Ray{o:player.position, d: Ray::from_angle(initial_angle + angle_increment * i as f64),})
	}


	for line in  lines{
		for (i,ray) in rays.iter().enumerate(){
			let (t,u) = ray_line_delta(ray,&line);
			if t > 0.0 && u > 0.0 && u < 1.0{

				let pct = t / RENDER_DISTANCE;

				let shading = if pct < 0.25{
				1 //this is for the closest
				}else if pct < 0.50 {
				2
				}
				else if pct < 0.75{
				3
				}
				else if pct < 1.0{
				4
				}
				else {0};
				
				load_line(buffer,i as usize,(RENDER_DISTANCE - t ) as usize,&shading);
				
			}
		}

	}



	}

fn display_player_coords(out:&mut Stdout,player: &Player)
{

		out.execute(cursor::MoveTo(2,2));
		write!(out,"x: {} ; y: {}, angle: {}",player.position.x,player.position.y,player.angle).unwrap();

}


fn main(){


	let mut player = Player::default();


	let mut lines :Vec<Line> = Vec::new();

	for i in 0..50{
		
		lines.push(
		Line{
			a: Vec2{x: (5 * i) as f64, y: 10.0},
			b: Vec2{x:(5.0 * i as f64 - 4.5) as f64, y: 10.0},

			}
		);
		
		
		}
		

	//this will be the buffer you actually make logic changes to
	let mut buffer: Vec<Vec<u8>> = vec![vec![0u8;SCREEN_MEASURES.0 as usize];SCREEN_MEASURES.1 as usize];
        //this is the buffer that only gets changed in the differences between it and the buffer to minimize
	//write operations on the console
	let mut pre_buffer: Vec<Vec<u8>> = vec![vec![0u8;SCREEN_MEASURES.0 as usize];SCREEN_MEASURES.1 as usize];

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

	display_player_coords(&mut stdout,&player);

	//this is the logic so that you can have
	// that sweet delta time available
	let now = Instant::now();
	let dt = now.duration_since(last);
	last = now;
	
	DELTA_TIME_NS.store(dt.as_nanos() as u64, Ordering::Relaxed);

	// Clear the buffer each frame
	for row in buffer.iter_mut() {
		for val in row.iter_mut() {
			*val = 0;
		}
	}

	render_fov(&mut buffer, &player, &lines);

	if event::poll(Duration::from_millis(0)).unwrap_or(false) {
		if let Ok(Event::Key(key)) = event::read(){
			if key.kind == KeyEventKind::Press {
				match key.code {
					KeyCode::Char('e') => player.rotate_left(),
					KeyCode::Char('q') => player.rotate_right(),
					KeyCode::Left =>player.move_left(),
					KeyCode::Right => player.move_right(),
					KeyCode::Up => player.move_up(),
					KeyCode::Down => player.move_down(),
					KeyCode::Esc => break,
					_ => {},
				}
			}
		}
	}

	display_buffer(&mut stdout,&mut buffer, &mut pre_buffer);

	//this is where the loop end btw
	}

	

	//stdout.execute(cursor::Show).unwrap();
	stdout.execute(terminal::LeaveAlternateScreen).unwrap();
	terminal::disable_raw_mode().unwrap();
	
	let _ = disable_raw_mode();
}



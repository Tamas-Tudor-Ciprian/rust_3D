mod vectors;
mod player;
mod delta_time;
mod geometry;


use vectors::*;
use player::*;
use geometry::*;


use crossterm::{
		cursor,
		terminal::{self,ClearType,enable_raw_mode, disable_raw_mode},
		event::{self, Event, KeyCode, KeyEventKind},
		ExecutableCommand,
		style::*,
		};



use std::io::{stdout, Write,Stdout};
use std::thread::sleep;


use rand::Rng;

use std::f64::consts::PI;


use std::time::Duration;

const SCREEN_MEASURES: (i32,i32) = (156,50);
const FOV: f64 = PI / 2.5 ;
const RENDER_DISTANCE : f64 = 25.0;
const SQUARE_SIZE : f64 = 3.0;
const SQUARE_DISTANCE : f64 = 0.0;







//this has been deprecated might remove later
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

fn display_minimap(out: &mut Stdout, minimap : &Vec<&str>, player : &Player) {


	let minimap_origin = (50,0); //subject to change
	let mut i = 0;
	for row in minimap{

		out.execute(cursor::MoveTo((SCREEN_MEASURES.0 - minimap_origin.0).try_into().unwrap(),(i + minimap_origin.1).try_into().unwrap()));
		write!(out,"{}",row);
		out.flush();
		i += 1;
	}

	//display the player

	let normalized_player_coords = ((player.position.x / 3.0) as i32,(player.position.y / 3.0) as i32);

	out.execute(cursor::MoveTo((SCREEN_MEASURES.0 -minimap_origin.0 + normalized_player_coords.0).try_into().unwrap(),
					(normalized_player_coords.1 + minimap_origin.1).try_into().unwrap()));
	write!(out,"{}",'P');

	



}


//this function loads a line into the buffer
fn load_line(buffer :&mut Vec<Vec<(u8,u8)>>,x : usize, len : usize, val : (u8,u8)){

	let row_count = buffer.len(); // number of rows (SCREEN_MEASURES.1)
	let len = len.min(row_count);  // clamp so we don't exceed screen height
	if len == 0 { return; }
	let start_point : usize = row_count/2 - len/2;

	if val.0 < buffer[row_count/2][x].0 || buffer[row_count/2][x].0 == 0{
		for i in 0..len{
			// write vertically: row varies, column is fixed
			buffer[i + start_point][x] = val;
			}
	}

	}



//this function actually displays the buffer only changing on the screen the differences between the buffer and the prebuffer
fn display_buffer(out: &mut Stdout,buffer: &mut Vec<Vec<(u8,u8)>>, pre_screen: &mut Vec<Vec<(u8,u8)>>){

	for (i,line) in buffer.iter().enumerate(){
		for (j,val) in line.iter().enumerate(){
			if *val != pre_screen[i][j]{
				pre_screen[i][j] = *val;
				out.execute(cursor::MoveTo(j as u16,i as u16));
		
				match val {
					(1,0) => write!(out,"{}","█".blue()).unwrap(),
					(2,0) => write!(out,"{}","▓".blue()).unwrap(),
					(3,0) => write!(out,"{}","▒".blue()).unwrap(),
					(4,0) => write!(out,"{}","░".blue()).unwrap(),
					(1,1) => write!(out,"{}","█".with(Color::White)).unwrap(),
					(2,1) => write!(out,"{}","▓".with(Color::White)).unwrap(),
					(3,1) => write!(out,"{}","▒".with(Color::White)).unwrap(),
					(4,1) => write!(out,"{}","░".with(Color::White)).unwrap(),
					_ => write!(out," ").unwrap(),

				}


			}
	
		}

}
}

fn render_fov(buffer: &mut Vec<Vec<(u8,u8)>>,player : &Player, lines : &Vec<Line>){

	let mut rays: Vec<Ray> = Vec::new();


	let initial_angle = player.angle - FOV/2.0;
	let angle_increment = FOV/SCREEN_MEASURES.0 as f64;

	for i in 0..SCREEN_MEASURES.0 {
		
		rays.push(Ray{o:player.position, d: Ray::from_angle(initial_angle + angle_increment * i as f64),})
	}


	for line in  lines{
		for (i,ray) in rays.iter().enumerate(){
			let (mut t,u) = ray_line_delta(ray,&line);
			if t > 0.0 && u > 0.0 && u < 1.0{
				
				let ray_angle = initial_angle + angle_increment  * i as f64;

				t = t * (ray_angle - player.angle).cos();

				let pct = t / RENDER_DISTANCE;


				let mut shading = if pct < 0.25{
				(1,0) //this is for the closest
				}else if pct < 0.50 {
				(2,0)
				}
				else if pct < 0.75{
				(3,0)
				}
				else if pct < 1.0{
				(4,0)
				}
				else {(0,0)};

				if u < 0.025 || u > 0.975{shading.1 = 1;}

				let wall_height = (SCREEN_MEASURES.1 as f64  * SQUARE_SIZE / t ) as usize;

				if shading.0 == 0 {continue;}	
				load_line(buffer,i as usize,wall_height ,shading);
				
			}
		}

	}



	}

fn get_lines_from_char_maze(maze : &Vec<&str>) -> Vec<Line>{

	let mut lines: Vec<Line> = Vec::new();


	for (i,line) in maze.iter().enumerate() {
		for (j,character) in line.chars().enumerate(){
			if character == '#'{
			//this are the lines that make up a square
			let starting_point = 0.0 ;
		

			let x_move = j as f64 * SQUARE_SIZE + SQUARE_DISTANCE;
			let y_move = i as f64 * SQUARE_SIZE + SQUARE_DISTANCE;

			//lower line
			lines.push(
			Line{
				a: Vec2{x: starting_point + x_move, y:  starting_point + y_move},
				b: Vec2{x:SQUARE_SIZE + x_move, y: starting_point + y_move},

			}
				);
			//leftside line
			lines.push(
			Line{
				a: Vec2{x: starting_point + x_move, y:  starting_point + y_move},
				b: Vec2{x:starting_point + x_move, y:SQUARE_SIZE + y_move},

			}
				);
			//upper line 
			lines.push(
			Line{
				a: Vec2{x: starting_point + x_move, y:  SQUARE_SIZE + y_move},
				b: Vec2{x:SQUARE_SIZE + x_move, y: SQUARE_SIZE + y_move},

			}
				);
			//rightside line
			lines.push(
			Line{
				a: Vec2{x: SQUARE_SIZE + x_move, y:  SQUARE_SIZE + y_move},
				b: Vec2{x:SQUARE_SIZE + x_move, y: starting_point + y_move},

			}
				);
				}	
			}

		}

	lines


	}

fn display_player_coords(out:&mut Stdout,player: &Player)
{

		out.execute(cursor::MoveTo(2,2));
		write!(out,"x: {} ; y: {}, angle: {}",player.position.x,player.position.y,player.angle).unwrap();

}


struct Direction{
	up: bool,
	down:bool,
	left:bool,
	right:bool,
}

impl Default for Direction{

	fn default()->Self{
	Self{
		up:true,
		down:true,
		left:true,
		right:true,
	}

	}

}


impl Direction{

pub fn get_direction_block(&mut self,  player: &Player, lines: &Vec<Line>){

	let circle = Circle{o:player.position, r : 1.0};

	for line in lines{

		let dir = circle_line_dir(&circle, line);

		
		if dir != None{
			let dir_result = dir.unwrap();
			let player_dir = Vec2::from_angle(&player.angle);
			let normalized_dir = Vec2{x: dir_result.x * player_dir.x + dir_result.y * player_dir.y,
						 y: -dir_result.x * player_dir.y + dir_result.y * player_dir.x};
			if normalized_dir.x < 0.0 {
				self.up = false;
			}
			if normalized_dir.x > 0.0 {
				 self.down = false;
			}			
			if normalized_dir.y < 0.0 {
				self.right = false;
			}
			if normalized_dir.y > 0.0 {
				self.left = false;
			}

		}


	
		
	}


}

}


fn main(){

	let minimap = vec![
			"#####################",
			"#                   #",
			"#                   #",
			"#      ########     #",
			"#                   #",
			"#                   #",
			"#     #########     #",
			"#                   #",
			"############## ######",
			];

	let mut player = Player::default();

	player.position = Vec2{x: 6.0,y: 6.0};



	let lines = get_lines_from_char_maze(&minimap);


	//this will be the buffer you actually make logic changes to
	let mut buffer: Vec<Vec<(u8,u8)>> = vec![vec![(0u8,0u8);SCREEN_MEASURES.0 as usize];SCREEN_MEASURES.1 as usize];
        //this is the buffer that only gets changed in the differences between it and the buffer to minimize
	//write operations on the console
	let mut pre_buffer: Vec<Vec<(u8,u8)>> = vec![vec![(0u8,0u8);SCREEN_MEASURES.0 as usize];SCREEN_MEASURES.1 as usize];

	let _ = enable_raw_mode();

	let mut stdout = stdout();

	stdout.execute(terminal::Clear(ClearType::All)).unwrap();



	//make_frame(&mut stdout);


	//this is not really neccessary
	
	sleep(Duration::from_millis(50));




	// this be the main game loop
	loop{


	delta_time::store();

	
	display_minimap(&mut stdout, &minimap, &player);
	display_player_coords(&mut stdout,&player);

	// Clear the buffer each frame
	for row in buffer.iter_mut() {
		for val in row.iter_mut() {
			*val = (0,0);
		}
	}

	render_fov(&mut buffer, &player, &lines);

	if event::poll(Duration::from_millis(0)).unwrap_or(false) {
		if let Ok(Event::Key(key)) = event::read(){
			if key.kind == KeyEventKind::Press {

				let mut dir_block = Direction::default();

				dir_block.get_direction_block(&player,&lines);

				match key.code {
					KeyCode::Char('e') => player.rotate_left(delta_time::get()),
					KeyCode::Char('q') => player.rotate_right(delta_time::get()),
					k if k == KeyCode::Char('a') && dir_block.left =>player.move_left(delta_time::get()),
					k if k == KeyCode::Char('d') && dir_block.right => player.move_right(delta_time::get()),
					k if k ==KeyCode::Char('w') && dir_block.up => player.move_up(delta_time::get()),
					k if k == KeyCode::Char('s')&& dir_block.down => player.move_down(delta_time::get()),
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



use std::thread::sleep;
use std::time::Duration;



const WALL:&str = "#";
const EMPTY:&str = " ";
const HEIGHT:usize = 8;
const WIDTH:usize = 16;



fn maze_boundary(width: usize, height : usize,maze: &mut Vec<String>) {



	let mut output = maze.clone();

	for i in 0..height{
		let mut row= "".to_string();
		for j in 0..width{
				let ch = match (i,j){
					(0,_) => WALL,
					(_,0) => WALL,
					(h,_) if h == height - 1 => WALL,
					(_,w) if w == width - 1 => WALL,
					_ => EMPTY,					
					   };

				row += ch;			
					


					}
		output.push(row);
	

				}


	*maze = output;

}

fn print_maze(height: usize, maze : &Vec<String>){


	for i in 0..height{
		println!("{}",maze[i]);
	}
}

		
fn add_maze_



						


fn main(){


	let mut step = 0;


	let mut maze = Vec::new();

	println!("Generating the maze...");
	sleep(Duration::from_secs(1));
	//this will clear the screen
	print!("\x1B[2J\x1B[H");



	maze_layer_gen(HEIGHT,WIDTH,&mut maze);
	println!("Step {}", step);
	print_maze(HEIGHT,&maze);				
}














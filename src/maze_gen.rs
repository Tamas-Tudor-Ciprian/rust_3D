use std::thread::sleep;
use std::time::Duration;


const HEIGHT:usize = 8;
const WIDTH:usize = 16;



fn lcg(seed:&mut u64) ->u64{
	*seed = seed.wrapping_mul(6364136223846793005).wrapping_add(543251323);
	*seed
}


fn rand_range(seed: &mut u64, low:u64, high: u64) -> u64 {
	low + (lcg(seed) % (high - low))
}



fn maze_boundary(width: usize, height : usize,maze: &mut Vec<String>) {



	let mut output = maze.clone();

	for i in 0..height{
		let mut row= "".to_string();
		for j in 0..width{
				let ch = match (i,j){
					(0,_) => WALL,
					(_,0) => WALL,
					(h,_) if h == (height - 1) => WALL,
					(_,w) if w == (width - 1) => WALL,
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

fn matrix(maze: &Vec<String>) -> Vec<Vec<char>> {
    maze.iter()
        .map(|row| row.chars().collect()).collect()
}


fn printable(maze: &Vec<Vec<char>>) -> Vec<String> {
	
	let mut output = Vec::new();


	for row in maze {
		let mut row_str = "".to_string();
		for ch in row{
			row_str += &ch.to_string();
		}
		output.push(row_str);
	}

	output



}

		
fn add_maze_level(maze: Vec<String>,seed : &mut u64) -> Vec<String>{


	let WALL : char = '#';
	let EMPTY : char = ' ';

	let mut matrix = matrix(&maze);

	let height = matrix.len();
	let width = matrix[0].len();

	for i in 1..(height-1){
		for j in 1..(width-1){
			
			if rand_range(seed,0,4) == 1 && matrix[i][j] == ' '{
			
			//this is the pattern that does all the magic for what I intended for this file
			//the matrix elements in the tuple represent the cells bordering the current one
			matrix[i][j] = match (matrix[i-1][j-1], //1
					      matrix[i-1][j],   //2
					      matrix[i-1][j+1], //3
					      matrix[i][j-1],   //4
					      matrix[i][j+1],   //5
					      matrix[i+1][j-1], //6
					      matrix[i+1][j],   //7
					      matrix[i+1][j+1]){//8
							(_,WALL,_,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY) => WALL,
							(_,EMPTY,EMPTY,WALL,EMPTY,_,EMPTY,EMPTY) => WALL,
							(EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,_,WALL,_) => WALL,
							(EMPTY,EMPTY,_,EMPTY,WALL,EMPTY,EMPTY,_) => WALL,
							_ => EMPTY,
						};
						}

		
					}

				}


	return printable(&matrix);


}


fn main(){


	let mut step = 0;


	let mut maze = Vec::new();

	let mut seed = 34527654321;



	println!("Generating the maze...");
	sleep(Duration::from_secs(1));
	//this will clear the screen
	print!("\x1B[2J\x1B[H");



	maze_boundary(WIDTH,HEIGHT,&mut maze);
	println!("Step {}", step);
	print_maze(HEIGHT,&maze);
	sleep(Duration::from_secs(1));


	print!("\x1B[2J\x1B[H");
	
	maze = add_maze_level(maze,&mut seed);
	step +=1;
	println!("Step {}", step);
	print_maze(HEIGHT,&maze);
	sleep(Duration::from_secs(1));
	print!("\x1B[2J\x1B[H");
	
	maze = add_maze_level(maze,&mut seed);
	step +=1;                
	println!("Step {}", step);
	print_maze(HEIGHT,&maze);


	sleep(Duration::from_secs(1));
	print!("\x1B[2J\x1B[H");
	
	maze = add_maze_level(maze,&mut seed);
	step +=1;                
	println!("Step {}", step);
	print_maze(HEIGHT,&maze);



}

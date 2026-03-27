use crate::vectors::*;



pub struct Circle{
	pub o: Vec2,
	pub r: f64,
}





pub struct Ray{
	pub o: Vec2,
	pub d: Vec2,
	}

impl Ray{
	pub fn from_angle(angle: f64) -> Vec2{
		Vec2{x:angle.cos(), y:angle.sin()}
	}
}



pub struct Line{
	pub a: Vec2,
	pub b: Vec2,
}


impl Line{
	pub fn get_direction_vec(&self) -> Vec2{
		self.b - self.a
}

pub fn ray_line_delta(r: &Ray, l: &Line) -> (f64,f64) {

	let e = l.b -l.a;
	
	let t = ((l.a - r.o) * e) / (r.d * e);
	let u = ((l.a - r.o) * r.d) / (r.d * e);

	(t,u)

}

pub fn circle_line_intersection( c: &Circle, l: &Line) -> bool {


	let line_direction = l.get_direction_vec();
	
	//this is the vector from the center of the circle to the "origin" of the line
	let cicle_line = 


	let a = line_direction.x * line_direction.x + line_direction.y * line_direction.y;
	let b = 2.0 * 


}

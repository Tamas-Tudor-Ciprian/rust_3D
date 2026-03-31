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


	//so basically you can get a second degree polynomial that allows you to solve the intersection by doing this:

	//create the parameters:

	let a = 1;
	let b = 1;
	let c = 1;


	let (t1, t2) = quadratic(a,b,c);

	

}


pub fn quadratic (a : f64, b : f64 , c : f64) -> Result<(f64,f64)>
{
	let delta = b * b - 4 * a * c;

	if delta < 0
	{
		return None;
	}

	let x1 = (-b + sqrt(delta)) / 2 * a;

	let x2 = (-b - sqrt(delta)) / 2 * a;
}

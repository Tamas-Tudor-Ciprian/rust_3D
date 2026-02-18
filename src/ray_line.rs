use crate::vectors::*;



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


pub fn ray_line_delta(r: &Ray, l: &Line) -> (f64,f64) {

	let e = l.b -l.a;
	
	let t = ((l.a - r.o) * e) / (r.d * e);
	let u = ((l.a - r.o) * r.d) / (r.d * e);

	(t,u)

}




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


impl Line
{
	pub fn get_direction_vector(&self) -> Vec2{
		self.b - self.a
	}


        pub fn get_slope(&self) -> f64
        {
            let dir = self.get_direction_vector();

            let slope = dir.y / dir.x;
		
	    slope
	
        }

        pub fn point_from_param(&self,t : f64) ->Vec2{

            let slope = self.get_slope();

	    let p_x = t + self.a.x;
	    let p_y = t * slope + self.a.y;
            
            Vec2{ x : p_x, y : p_y}

        }

	

}
pub fn ray_line_delta(r: &Ray, l: &Line) -> (f64,f64) {

	let e = l.b -l.a;
	
	let t = ((l.a - r.o) * e) / (r.d * e);
	let u = ((l.a - r.o) * r.d) / (r.d * e);

	(t,u)

}

pub fn circle_line_intersection( c: &Circle, l: &Line) -> Option<(Vec2,Vec2)> {

	//this outta do it, hopefully needs testing

	//so basically you can get a second degree polynomial that allows you to solve the intersection by doing this:

	//create the parameters:

	let line_dir = l.get_direction_vector();

	let slope = line_dir.y/line_dir.x;

	let a = l.a.x - c.o.x;

	let b = l.a.y - c.o.y;

	//now for the variables we will actually use

	let alpha = 1.0+ slope * slope;
	let beta = 2.0 * (a + b * slope);
	let c = a * a + b * b - c.r *c.r;


	let intersection = quadratic(alpha,beta,c);


        if intersection == None
        {
            return None;
        }


	let (t1,t2) = intersection.unwrap();


        let p1 = l.point_from_param(t1);

        let p2 = l.point_from_param(t2);
	
        Some((p1,p2))	

}


pub fn quadratic (a : f64, b : f64 , c : f64) -> Option<(f64,f64)>
{
	let delta = b * b - 4.0 * a * c;

	if delta < 0.0
	{
		return None;
	}

	let x1 = (-b + delta.sqrt()) / (2.0 * a);

	let x2 = (-b - delta.sqrt()) / (2.0 * a);

	return Some((x1,x2));
}


#[cfg(test)]
mod tests {
	use super::*;


	const EPSILON: f64 = 1e-10;


	fn assert_vec2_near(actual: Vec2, expected: Vec2)
	{
		assert!(
		(actual.x - expected.x).abs() < EPSILON,
		"x coords differ: actual {}, expected {}",actual.x,expected.x
		);
		assert!(
		(actual.y - expected.y).abs() < EPSILON,
		"y coords differ: actual {}, expected {}",actual.y,expected.y
		);
	}

	#[test]
	fn test_quad(){
		let result = quadratic(1.0,-5.0,6.0);
		assert!(result == Some((2.0,3.0)) || result == Some((3.0,2.0)));

	}

	#[test]
	fn test_circle_line_intersection1(){
	
		let circle = Circle{o:Vec2{x:1.0, y:1.0},r:1.0};
		
		let line = Line{a:Vec2{x:0.0,y:0.0},b:Vec2{x:2.0,y:2.0}};

		let result = circle_line_intersection(&circle,&line).unwrap();
    
                let p1 = result.0;
                let p2 = result.1;

		let expected1 = Vec2{x:	0.29289, y:0.29289};
		let expected2 = Vec2{x:1.70711, y:1.70711};

		assert_vec2_near(p1,expected1);
		assert_vec2_near(p2,expected2);

	}




}




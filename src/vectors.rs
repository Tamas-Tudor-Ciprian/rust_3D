use std::ops::{Add,Sub,Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
	pub x:f64,
	pub y:f64,
}


impl Add for Vec2 {
	type Output = Vec2;

	fn add(self, rhs:Vec2) -> Vec2 {
		Vec2{
			x: self.x + rhs.x,
			y: self.y + rhs.y,


			}


		}


	}

impl Sub for Vec2{
	type Output = Vec2;
	
	fn sub(self,rhs:Vec2) -> Vec2 {
		Vec2{
			x: self.x - rhs.x,
			y: self.y - rhs.y,

			}

		}



}


impl Mul for Vec2{
	type Output = f64;

	fn mul(self, rhs:Vec2) -> f64{
		self.x * rhs.y - self.y * rhs.x
		}


	}

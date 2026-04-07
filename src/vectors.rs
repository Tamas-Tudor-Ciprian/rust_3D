use std::ops::{Add,Sub,Mul,BitAnd};

#[derive(Debug, Copy, Clone,PartialEq)]
pub struct Vec2 {
	pub x:f64,
	pub y:f64,
}

impl Vec2{
	pub fn from_angle(theta: &f64) ->Self{
		Self{
		x: theta.cos(),
		y: theta.sin(),
		}

	}


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

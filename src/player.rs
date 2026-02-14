use crate::vectors::*;
use std::f64::consts::PI;

const PLAYER_SPEED : f64 = 200.0;
const PLAYER_ROTATION_SPEED : f64 = 25.0;



pub struct Player {
        pub position : Vec2,
        pub angle : f64,
        pub speed : f64,
        }


impl Player{
        pub fn rotate_left(&mut self,delta_time : f64){
                self.angle += PLAYER_ROTATION_SPEED * delta_time;
                }
        pub fn rotate_right(&mut self, delta_time: f64){
                self.angle -= PLAYER_ROTATION_SPEED * delta_time;
                }

        pub fn move_right(&mut self, delta_time: f64){
                self.position.x += (self.angle + PI/2.0).cos() * self.speed * delta_time;
                self.position.y += (self.angle + PI/2.0).sin() * self.speed * delta_time;
        }
        pub fn move_left(&mut self, delta_time: f64){
                self.position.x += (self.angle - PI/2.0).cos() * self.speed * delta_time;
                self.position.y += (self.angle - PI/2.0).sin() * self.speed * delta_time;
                        }
        pub fn move_up(&mut self, delta_time: f64){
                self.position.x += self.angle.cos() * self.speed * delta_time;
                self.position.y += self.angle.sin() * self.speed * delta_time;
                }
        pub fn move_down(&mut self, delta_time: f64){

                self.position.x -= self.angle.cos() * self.speed * delta_time;
                self.position.y -= self.angle.sin() * self.speed * delta_time; }



        }


impl Default for Player{
        fn default() -> Self{
                Self{position : Vec2{x : 0.0, y: 0.0},
                angle : 0.0,
                speed : PLAYER_SPEED,
                }}
        }

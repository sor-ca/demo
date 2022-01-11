use std::time::{Duration, Instant};           
use std::convert::TryFrom;
//use macroquad_tiled_redux::animation::AnimationFrame;
use std::iter::Iterator;
#[derive(Clone)]
pub struct AnimationFrame {
    tile_id: u32,
    duration: Duration,
}
pub struct AnimationTemplate {
    pub name: String,
    //pub gid: u32,
    pub frames: Vec<AnimationFrame>,
    //pub ordering: u8,
    //pub max_compression: u32,
    //pub blocks_turn: bool,
    //pub cancel_frame: Option<u32>,
}
struct AnimationInstance {
    //pub state: TiAnimationState,
    pub frames: Vec<AnimationFrame>,
    pub duration: Duration,
    pub movement: (i32, i32),
}
//impl AnimationInstance {
    //pub fn new(start_time: Instant, template: &AnimationTemplate) -> Self {
        //Self::new_movement(start_time, template, (0, 0))
    //}
    //pub fn new_movement(start_time: Instant, template: &AnimationTemplate, movement: (i32, i32)) -> Self {
        //let total_ticks = template.frames.iter().map(|it| it.duration.as_millis()).sum();

        //Self {
            //state: TiAnimationState::new(template.gid, start_time, false),
            //duration: Duration::from_millis(total_ticks),
            //frames: template.frames.clone(),
            //movement,
        //}
    //}
//}
pub struct AnimationController {
    animation_start: Option<Instant>,
    animations: Vec<AnimationInstance>,
    //idle_interval: Option<Duration>,
    //idle_animations: Vec<TiAnimationState>,
}
impl AnimationController {
    pub fn update(&mut self, time: Instant, template: &AnimationTemplate) {
        todo!()
    }
    pub fn get_frame(&self, time: Instant, template: &AnimationTemplate) -> Option<(u32, (f32, f32))> {
        todo!()
    }
    pub fn add_animation(&mut self, start_time: Instant, template: &AnimationTemplate, movement: (i32, i32)) {
        //let instance = AnimationInstance::new_movement(start_time, template, movement);
        //self.animations.push(instance);
        todo!()
    }
}
fn get_tileid(start_time: Instant, finish_time: Instant, frames: &Vec<AnimationFrame>) -> u32 {
    let mut sum=frames.iter().map(|x|x.duration.as_millis()).sum();
    let time = finish_time-start_time;
    let mut time = time.as_millis();
    //let sum=frames.iter().map(|sum, &x|sum.checked_add(x.duration).unwrap());
    time=time%sum;
    let mut tileid = 0;
    for i in frames.iter().rev() {
        sum -=i.duration.as_millis();
        if time>sum {
            tileid=i.tile_id;
            break;
        }
    }
    tileid

}
fn get_position(start_time: Instant, finish_time:Instant, start_position:(i32,i32), movement: (i32,i32)) -> (i32,i32) {
    let time=finish_time - start_time;
    let time = i32::try_from(time.as_millis()).unwrap();
    let x=start_position.0 + (movement.0*time)/1000;
    let y=start_position.1 + (movement.1*time)/1000;
    (x,y)
}


fn main(){
        let ms = Duration::from_millis(1);
        let frames: Vec<AnimationFrame> = vec![
            AnimationFrame { tile_id: 1, duration: ms*100, },
            AnimationFrame { tile_id: 2, duration: ms*200, },
            AnimationFrame { tile_id: 3, duration: ms*400, },
            AnimationFrame { tile_id: 4, duration: ms*300, },
        ];
        let start_time=Instant::now();
        let finish_time=start_time+99*ms;
        let tile_id = get_tileid(start_time,finish_time,&frames);
        let position = get_position(start_time,finish_time,(0,0),(1000,10));
        println!("{}", tile_id);
        println!("{},{}",position.0,position.1);
}
//#[cfg(test)]
//mod tests {
    //use coarsetime::{Duration, Instant};
    //use macroquad_tiled_redux::animation::AnimationFrame;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    //#[test]
    //fn test_frame0() {
        //let ms = Duration::from_millis(1);
        //let mut controller = AnimationController::new();
        //let mut now = Instant::now();
        //// total duration: 1000 ms
        //let frames: Vec<AnimationFrame> = vec![
            //AnimationFrame { tile_id: 1, duration: ms*100, },
            //AnimationFrame { tile_id: 2, duration: ms*200, },
            //AnimationFrame { tile_id: 3, duration: ms*400, },
            //AnimationFrame { tile_id: 4, duration: ms*300, },
        //];
        //let template = AnimationTemplate {
            //name: "dummy".to_string(),
            //gid: 1,
            //frames,
            //ordering: 0,
            //max_compression: 0,
            //blocks_turn: false,
            //cancel_frame: None,
        //};

        //controller.add_animation(now, &template, (1000, 10));

        //controller.update(now);
        //let frame_at_0 = controller.get_frame(now)
            //.expect("Frame expected");
        //assert_eq!(frame_at_0.0, 1);
        //assert_eq!(frame_at_0.1, (0.0, 0.0));

        //now += ms * 99;
        //controller.update(now);
        //let frame_at_99 = controller.get_frame(0,99, frames)
            //.expect("Frame expected");
        //assert_eq!(frame_at_99.0, 1);
        //assert_eq!(frame_at_99.1, (99.0, 0.0));

        //now += ms;
        //let frame_at_100 = controller.get_frame(0,100, frames);
            //.expect("Frame expected");
        //// it's time for tile 2, b/c first frame duration is 100ms
        //assert_eq!(frame_at_100.0, 2);
        //assert_eq!(frame_at_100.1, (100.0, 1.0));

        //// and so on.

        //// Also test if the state is valid empty state after all frames are gone.
    //}

//}

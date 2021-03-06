use coarsetime::{Duration, Instant};
use std::vec::Vec;
use std::iter::Iterator;

#[derive(Clone, Copy, Debug)]
pub struct AnimationFrame {
    tile_id: u32,
    duration: Duration,
}

#[derive(Clone, Copy)]
pub struct AnimatedSpriteState {
    pub animation_id: u32,
    /// Current frame
    pub frame: u32,
    /// Time the last current frame (should have) started at.
    pub frame_start: Instant,
    pub playing: bool,
}

impl AnimatedSpriteState {
    
    pub fn new(current_animation: u32, start: Instant, playing: bool) -> Self {
        Self {
            animation_id: current_animation,
            frame_start: start,
            frame: 0,
            playing,
        }
    }
}

pub struct AnimationTemplate {
    pub name: String,
    pub gid: u32,
    pub frames: Vec<AnimationFrame>,
    //pub ordering: u8,
    pub max_compression: u32,
    //pub blocks_turn: bool,
    //pub cancel_frame: Option<u32>,
}

#[derive(Clone)]
struct AnimationInstance {
    pub state: AnimatedSpriteState,
    pub frames: Vec<AnimationFrame>,
    pub duration: Duration,
    pub movement: (i32, i32),
    pub start_position: (f32, f32),
    pub max_compression: u32,
    pub is_compressed: bool,
}

impl AnimationInstance {
    
    pub fn new(start_time: Instant, template: &AnimationTemplate) -> Self {
        Self::new_movement(start_time, template, (0, 0),(0.0, 0.0))
    }

    pub fn new_movement(start_time: Instant, template: &AnimationTemplate, movement: (i32, i32), start_position: (f32,f32)) -> Self {
        let total_ticks = template.frames.iter().map(|it| it.duration.as_ticks() as u64).sum();
        Self {
            state: AnimatedSpriteState::new(template.gid, start_time, false),
            duration: Duration::from_ticks(total_ticks),
            frames: template.frames.clone(),
            movement,
            start_position,
            max_compression: template.max_compression,
            is_compressed: false,
        }
    }
    pub fn compressed(&mut self, current_time: Instant) {
            let frames = self.frames.clone();
            let mut new_frames: Vec<AnimationFrame> = vec![];
            let mut start = self.state.frame_start.clone(); 
            let mut new_start = self.state.frame_start.clone(); 
            for i in &frames {
                if start+i.duration <= current_time {
                    new_start = start;
                }
                if start+i.duration > current_time {
                    let f =  AnimationFrame {
                        tile_id: i.tile_id,
                        duration: i.duration*self.max_compression/100,
                    };
                    new_frames.push(f);
                }
                start += i.duration;
            }
            let new_duration = new_frames.iter().map(|it| it.duration.as_ticks() as u64).sum();
            let k = (self.duration.as_ticks() as u64 * self.max_compression as u64 / (new_duration * 100)) as f32;
            let new_movement = ((self.movement.0 as f32 / k) as i32, (self.movement.1 as f32 / k) as i32);
            //self.state.frame_start = self.state.frame_start + (self.duration - Duration::from_ticks((new_duration as f32 * k) as u64));
            //?????? ??????
            self.state.frame_start = new_start;
            //?????? ??????
            //self.state.frame_start = current_time;
            self.frames = new_frames;
            self.duration = Duration::from_ticks(new_duration);
            self.movement = new_movement;
            self.is_compressed = true;
    }
}

pub struct AnimationController {
    animation_start: Option<Instant>,
    animations: Vec<AnimationInstance>,
    //idle_interval: Option<Duration>,
    //idle_animations: Vec<TiAnimationState>,
}

impl AnimationController {
    pub fn new() -> Self {
        // Create an empty instance.
        Self {
            animation_start: None,
            animations: vec![],
            //idle_interval: None,
            //idle_animations: vec![],
        }
    }

    pub fn update(&mut self, time: Instant) {
        if self.animations.len() != 0 {
            let mut animations = self.animations.clone();
            animations.retain(|i|i.state.frame_start + i.duration >= time);
            self.animations = animations;
        }   
    }

    pub fn get_compressed(&mut self, time: Instant) {
            let mut animations = self.animations.clone();
            for i in &mut animations {
                if !i.is_compressed {
                    i.compressed(time);
                }
            }
            self.animations = animations;
    }

    pub fn get_frame(&self, time: Instant) -> Option<(u32, (f32, f32))> {
        match self.animations.get(0) {
            Some(i) => {
                let instance = i;
                let tile_id = AnimationController::get_tile_id(time, instance);
                let position = AnimationController::get_position(time, instance);
                let frame:(u32, (f32, f32)) = (tile_id, position);
                return Some(frame);
            }
            None => None
        }
    }

    pub fn add_animation(&mut self, start_time: Instant, template: &AnimationTemplate, movement: (i32, i32)) {
        let mut new_start_time = start_time;
        let mut new_start_position: (f32, f32) = (0.0, 0.0);
        if self.animations.len() != 0 {
            let i = self.animations.last().unwrap();
            new_start_time = i.state.frame_start + i.duration;
            new_start_position = (i.start_position.0 + i.movement.0 as f32, i.start_position.1 + i.movement.1 as f32)
        }
        let instance = AnimationInstance::new_movement(new_start_time, template, movement, new_start_position);
        self.animations.push(instance);
    }

    pub fn add_animation_with_compression(&mut self, start_time: Instant, template: &AnimationTemplate, movement: (i32, i32)) {
        let mut new_start_time = start_time;
        let mut new_start_position: (f32, f32) = (0.0, 0.0);
        if self.animations.len() != 0 {
            self.get_compressed(start_time);
            let i = self.animations.last().unwrap();
            new_start_time = i.state.frame_start + i.duration;
            new_start_position = (i.start_position.0 + i.movement.0 as f32, i.start_position.1 + i.movement.1 as f32)
        }
        let instance = AnimationInstance::new_movement(new_start_time, template, movement, new_start_position);
        self.animations.push(instance);
    }

    fn get_tile_id(finish_time: Instant, instance: &AnimationInstance) -> u32 {    
        let frames = &instance.frames;
        let mut tile_id = 0;
        let start_time = instance.state.frame_start;
        let mut time = finish_time - start_time;
        for i in frames {
            if time < i.duration {
                tile_id = i.tile_id;
                break;
            }
            time -= i.duration;
        }
        tile_id
    }

    fn get_position(finish_time:Instant, instance: &AnimationInstance) -> (f32,f32) {
        let movement = instance.movement;
        let start_position = instance.start_position;
        let start_time = instance.state.frame_start;
        let duration = (finish_time - start_time).as_ticks() as f32;
        let total_duration = instance.duration.as_ticks() as f32;
        let x = start_position.0 as f32 + ((movement.0 as f32 * duration) / total_duration) as f32;
        let y = start_position.1 as f32 + ((movement.1 as f32 * duration) / total_duration) as f32;
        (x.round(), y.round())
    }
}

fn main() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_frame0() {
        let mut controller = AnimationController::new();
        let time_start = Instant::now();
        let now = time_start;
        let frames: Vec<AnimationFrame> = vec![
            AnimationFrame { tile_id: 1, duration: Duration::from_millis(100), },
            AnimationFrame { tile_id: 2, duration: Duration::from_millis(200), },
            AnimationFrame { tile_id: 3, duration: Duration::from_millis(400), },
            AnimationFrame { tile_id: 4, duration: Duration::from_millis(300), },
        ];
        let template = AnimationTemplate {
            name: "dummy".to_string(),
            gid: 1,
            frames,
            ////ordering: 0,
            max_compression: 0,
            ////blocks_turn: false,
            ////cancel_frame: None,
        };

        controller.add_animation(now, &template, (1000, 100));

        controller.update(now);
        let frame_at_0 = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_0.0, 1);
        assert_eq!(frame_at_0.1, (0.0, 0.0));

        let now = time_start + Duration::from_millis(90);
        println!("90ms: {}", (now - time_start).as_millis());
        controller.update(now);
        let frame_at_91 = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_91.0, 1);
        assert_eq!(frame_at_91.1, (90.0, 9.0));

        let now = time_start + Duration::from_millis(100);
        let frame_at_100 = controller.get_frame(now)
           .expect("Frame expected");
        //// it's time for tile 2, b/c first frame duration is 100ms
        assert_eq!(frame_at_100.0, 2);
        assert_eq!(frame_at_100.1, (100.0, 10.0));

        let now = time_start + Duration::from_millis(101);
        controller.update(now);
        let frame_at_101 = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_101.0, 2);
        assert_eq!(frame_at_101.1, (101.0, 10.0));

        let now = time_start + Duration::from_millis(151);
        controller.update(now);
        let frame_at_200 = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_200.0, 2);
        assert_eq!(frame_at_200.1, (151.0, 15.0));
        let now = time_start + Duration::from_millis(1000);
        controller.update(now);
        let frame_at_1000 = controller.get_frame(now);
        assert_eq!(frame_at_1000, None);
    }

    #[test]
    fn test_2_instances() {
        let mut controller = AnimationController::new();
        let time_start = Instant::now();
        let now = time_start;
        //// total duration: 1000 ms
        let frames: Vec<AnimationFrame> = vec![
            AnimationFrame { tile_id: 1, duration: Duration::from_millis(100), },
            AnimationFrame { tile_id: 2, duration: Duration::from_millis(200), },
            AnimationFrame { tile_id: 3, duration: Duration::from_millis(400), },
            AnimationFrame { tile_id: 4, duration: Duration::from_millis(300), },
        ];
        let template = AnimationTemplate {
            name: "dummy".to_string(),
            gid: 1,
            frames,
            ////ordering: 0,
            max_compression: 0,
            ////blocks_turn: false,
            ////cancel_frame: None,
        };

        controller.add_animation(now, &template, (1000, 100));
        let frames: Vec<AnimationFrame> = vec![
            AnimationFrame { tile_id: 5, duration: Duration::from_millis(100), },
            AnimationFrame { tile_id: 6, duration: Duration::from_millis(200), },
            AnimationFrame { tile_id: 7, duration: Duration::from_millis(400), },
            AnimationFrame { tile_id: 8, duration: Duration::from_millis(300), },
        ];
        let template = AnimationTemplate {
            name: "dummy".to_string(),
            gid: 1,
            frames,
            ////ordering: 0,
            max_compression: 0,
            ////blocks_turn: false,
            ////cancel_frame: None,
        };
        controller.add_animation(now, &template, (1000, 100));
        println!("{}", controller.animations.len());
        for i in &controller.animations {
            println!("{},{}", i.start_position.0,i.start_position.1);
        };
        controller.update(now);
        let frame_at_0 = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_0.0, 1);
        assert_eq!(frame_at_0.1, (0.0, 0.0));

        let now = time_start + Duration::from_millis(90);
        println!("90ms: {}", (now - time_start).as_millis());
        controller.update(now);
        let frame_at_91 = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_91.0, 1);
        assert_eq!(frame_at_91.1, (90.0, 9.0));

        let now = time_start + Duration::from_millis(100);
        let frame_at_100 = controller.get_frame(now)
           .expect("Frame expected");
        //// it's time for tile 2, b/c first frame duration is 100ms
        assert_eq!(frame_at_100.0, 2);
        assert_eq!(frame_at_100.1, (100.0, 10.0));

        let now = time_start + Duration::from_millis(1090);
        controller.update(now);
        let frame_at_1090 = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_1090.0, 5);
        assert_eq!(frame_at_1090.1, (1090.0, 109.0));

        let now = time_start + Duration::from_millis(1100);
        controller.update(now);
        let frame_at_1100  = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_1100.0, 6);
        assert_eq!(frame_at_1100.1, (1100.0, 110.0));

        let now = time_start + Duration::from_millis(2000);
        controller.update(now);
        let frame_at_2000 = controller.get_frame(now);
        assert_eq!(frame_at_2000, None);
    }

    #[test]
    fn test_3_instances_with_compression() {
        let mut controller = AnimationController::new();
        let time_start = Instant::now();
        let now = time_start;
        let frames: Vec<AnimationFrame> = vec![
            AnimationFrame { tile_id: 1, duration: Duration::from_millis(100), },
            AnimationFrame { tile_id: 2, duration: Duration::from_millis(200), },
            AnimationFrame { tile_id: 3, duration: Duration::from_millis(400), },
            AnimationFrame { tile_id: 4, duration: Duration::from_millis(300), },
        ];
        let template = AnimationTemplate {
            name: "dummy".to_string(),
            gid: 1,
            frames,
            ////ordering: 0,
            max_compression: 50,
            ////blocks_turn: false,
            ////cancel_frame: None,
        };
        controller.add_animation_with_compression(now, &template, (1000, 100));

        let now = time_start + Duration::from_millis(1);
        let frames: Vec<AnimationFrame> = vec![
            AnimationFrame { tile_id: 5, duration: Duration::from_millis(100), },
            AnimationFrame { tile_id: 6, duration: Duration::from_millis(200), },
            AnimationFrame { tile_id: 7, duration: Duration::from_millis(400), },
            AnimationFrame { tile_id: 8, duration: Duration::from_millis(300), },
        ];
        let template = AnimationTemplate {
            name: "dummy".to_string(),
            gid: 1,
            frames,
            ////ordering: 0,
            max_compression: 50,
            ////blocks_turn: false,
            ////cancel_frame: None,
        };
        controller.add_animation_with_compression(now, &template, (1000, 100));
        let now = time_start + Duration::from_millis(2);
        let frames: Vec<AnimationFrame> = vec![
            AnimationFrame { tile_id: 9, duration: Duration::from_millis(100), },
            AnimationFrame { tile_id: 10, duration: Duration::from_millis(200), },
            AnimationFrame { tile_id: 11, duration: Duration::from_millis(400), },
            AnimationFrame { tile_id: 12, duration: Duration::from_millis(300), },
        ];
        let template = AnimationTemplate {
            name: "dummy".to_string(),
            gid: 1,
            frames,
            ////ordering: 0,
            max_compression: 50,
            ////blocks_turn: false,
            ////cancel_frame: None,
        };
        controller.add_animation_with_compression(now, &template, (1000, 100));
        println!("{}", controller.animations.len());
        for i in &controller.animations {
            println!("{},{}", i.start_position.0,i.start_position.1);
        };
        for y in &controller.animations{
            for i in &y.frames {
                println!("{}", i.duration.as_millis());
            }
        }

        controller.update(now);
        let frame_at_0 = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_0.0, 1);
        //assert_eq!(frame_at_0.1, (0.0, 0.0));
        println!("position 3.0, 0.0 = {}, {}", frame_at_0.1.0, frame_at_0.1.1);

        let now = time_start + Duration::from_millis(45);
        controller.update(now);
        let frame_at_45 = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_45.0, 1);
        println!("position 90.0, 9.0 = {}, {}", frame_at_45.1.0, frame_at_45.1.1);
        //assert_eq!(frame_at_45.1, (90.0, 9.0));

        let now = time_start + Duration::from_millis(51);
        let frame_at_50 = controller.get_frame(now)
           .expect("Frame expected");
        assert_eq!(frame_at_50.0, 2);
        //assert_eq!(frame_at_50.1, (100.0, 10.0));
        println!("position 100.0, 10.0 = {}, {}", frame_at_50.1.0, frame_at_50.1.1);

        let now = time_start + Duration::from_millis(545);
        controller.update(now);
        let frame_at_545 = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_545.0, 5);
        //assert_eq!(frame_at_545.1, (1090.0, 109.0));
        println!("position 1090.0, 109.0 = {}, {}", frame_at_545.1.0, frame_at_545.1.1);

        let now = time_start + Duration::from_millis(551);
        controller.update(now);
        let frame_at_551  = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_551.0, 6);
        //assert_eq!(frame_at_600.1, (1100.0, 110.0));
        println!("position 1100.0, 110.0 = {}, {}", frame_at_551.1.0, frame_at_551.1.1);
        
        let now = time_start + Duration::from_millis(1090);
        controller.update(now);
        let frame_at_1090 = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_1090.0, 9);
        //assert_eq!(frame_at_590.1, (1090.0, 109.0));
        println!("position 2090.0, 209.0 = {}, {}", frame_at_1090.1.0, frame_at_1090.1.1);

        let now = time_start + Duration::from_millis(1103);
        controller.update(now);
        let frame_at_1103  = controller.get_frame(now)
            .expect("Frame expected");
        assert_eq!(frame_at_1103.0, 10);
        //assert_eq!(frame_at_600.1, (1100.0, 110.0));
        println!("position 2100.0, 210.0 = {}, {}", frame_at_1103.1.0, frame_at_1103.1.1);


        let now = time_start + Duration::from_millis(2000);
        controller.update(now);
        let frame_at_2000 = controller.get_frame(now);
        assert_eq!(frame_at_2000, None);
    }
}

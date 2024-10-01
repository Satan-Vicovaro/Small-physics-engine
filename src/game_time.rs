use sdl2:: TimerSubsystem;
use sdl2::Sdl;
use crate::DrawHandler;
use sdl2::render::Canvas;
pub struct GameTime   {
    timer_subsystem: TimerSubsystem,
    last_frame_time:u64,
    current_frame_time:u64,
    physic_ticks_clock:u64,
    physic_ticks:u64,
    overal_physic_ticks_counter:u64,
    fps:f64,
    frames_clock:u64,
    frames_counter:u64,
}

const TICS_PER_SECOND:u64 = 64;

#[allow(dead_code)]
impl GameTime {
    pub fn new(sdl_context:&Sdl) -> Self {
        return GameTime {
            timer_subsystem:sdl_context.timer().unwrap(),
            last_frame_time:0,
            current_frame_time:0,
            physic_ticks_clock:0,
            physic_ticks:0,
            overal_physic_ticks_counter:0,
            fps:0.0,
            frames_clock:0,
            frames_counter:0,
            //clock:0,
        }
    }
    pub fn count_time(&mut self) {

        self.last_frame_time = self.current_frame_time;   
        self.current_frame_time = self.timer_subsystem.performance_counter();
        if self.last_frame_time == 0 {
            return;
        } 
        let delta = self.current_frame_time - self.last_frame_time;

        //self.clock += delta;
        self.physic_ticks_clock += delta;
        self.frames_clock+=delta;
        self.physic_ticks = 0;

        let physic_treshold = self.timer_subsystem.performance_frequency()/TICS_PER_SECOND ;
        if  self.physic_ticks_clock / physic_treshold >= 1 {
            self.physic_ticks = self.physic_ticks_clock / physic_treshold;
            self.overal_physic_ticks_counter += self.physic_ticks;
            self.physic_ticks_clock %= physic_treshold;
        }

        let frames_treshold = self.timer_subsystem.performance_frequency();
        if  self.frames_clock / frames_treshold >= 1 {

            self.fps = (self.frames_counter as f64) / (self.frames_clock / frames_treshold) as f64 ;
            self.frames_clock %= frames_treshold;
            self.frames_counter=0;
        }
        self.frames_counter += 1;
    }

    pub fn get_phisic_ticks(& self) -> u64 {
        return self.physic_ticks;
    }
    pub fn get_timer_subsystem (& self) -> &TimerSubsystem{
        return &self.timer_subsystem
    }
    pub fn get_delta_t_val (& self) -> f64{ 
        return 1.0/TICS_PER_SECOND as f64;
    }
    pub fn get_overal_physic_ticks_counter(& self) -> u64 {
        return self.overal_physic_ticks_counter;
    }
    pub fn timer(&self,miliseconds:u64) -> bool {
        let physic_ticks =miliseconds / (TICS_PER_SECOND * 1000);
        if self.overal_physic_ticks_counter % physic_ticks == 0 {
            //println!("zwracam prawdę");
            return true;
        }
        return false;
    }
    pub fn display_info(&self, canvas: &mut Canvas<sdl2::video::Window>, draw_handler:&mut DrawHandler) -> Result<(), String>  {
        let text = format!("Fps:{0}\nPhysic ticks counter:{1}\n",self.fps,self.overal_physic_ticks_counter);
        return draw_handler.draw_text(&text, (0.0,0.0), canvas);
    }
}
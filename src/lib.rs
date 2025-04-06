use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct State {
    pub acceleration: i32,
    pub speed: i32,
    pub position: i32,
    pub position_goal_start: i32,
    pub position_goal_end: i32,
    pub won: bool,
    pub lost: bool
}

#[wasm_bindgen]
pub struct Camera {
    pub screen_size: i32,
    pub world_size: i32,
    pub world_position: i32
}

#[wasm_bindgen]
impl Camera {
    #[wasm_bindgen(constructor)]
    pub fn new(screen_size: i32, world_size: i32) -> Camera {
        Camera {
            screen_size,
            world_size,
            world_position: 0
        }
    }
    pub fn project(&self, world_position: i32) -> i32{
        self.screen_size * (self.world_position + (self.world_size / 2) - world_position)  / self.world_size
    }
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new() -> State {
        State {
            position_goal_start: 9000,
            position_goal_end: 10000,
            position: 500,
            ..State::default()
        }
    }
}

impl Default for State {
    fn default() -> State {
        State {
            acceleration: 0,
            speed: 0,
            position: 0,
            position_goal_start: 0,
            position_goal_end: 0,
            lost: false,
            won: false
        }
    }
}

#[wasm_bindgen]
pub fn update(current_state: State, throttle: i32) -> State{
    State {
        acceleration: throttle,
        speed: (current_state.speed + current_state.acceleration).clamp(0, i32::MAX),
        position: current_state.position + current_state.speed,
        lost: current_state.position > current_state.position_goal_end,
        won: current_state.speed == 0 && current_state.position > current_state.position_goal_start && current_state.position < current_state.position_goal_end,
        position_goal_start: current_state.position_goal_start,
        position_goal_end: current_state.position_goal_end,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn throttle_should_set_acceleration_when_initially_zero() {
        let current_state = State {
            ..Default::default()
        };
        let new_state = update(current_state, 1);
        assert_eq!(1, new_state.acceleration);
    }

    #[test]
    fn throttle_should_not_increment_acceleration_if_already_to_maximum() {
        let current_state = State {
            acceleration: 1,
            ..Default::default()
        };
        let new_state = update(current_state, 1);
        assert_eq!(1, new_state.acceleration);
    }

    #[test]
    fn no_throttle_should_update_speed_with_current_acceleration(){

        let current_state = State {
            acceleration: 1,
            speed: 0,
            ..Default::default()
        };
        let new_state = update(current_state, 0);
        assert_eq!(1, new_state.speed);
    }

    #[test]
    fn no_throttle_should_update_position_from_speed (){
        let current_state = State {
            speed: 1,
            ..Default::default()
        };

        let new_state = update(current_state, 0);
        assert_eq!(1, new_state.position);
    }

    #[test]
    fn no_throttle_should_add_current_speed_to_current_position () {
        let current_state = State {
            speed: 1,
            position: 1,
            ..Default::default()
        };

        let new_state = update(current_state, 0);
        assert_eq!(2, new_state.position);
    }

    #[test]
    fn game_is_lost_if_over_position_goal_end() {
        let current_state = State {
            position: 2,
            position_goal_end: 1,
            lost: false,
            ..Default::default()
        };

        let new_state = update(current_state, 0);
        assert_eq!(true, new_state.lost);
    }

    #[test]
    fn game_is_not_lost_if_position_is_before_position_goal_end(){
        let current_state = State {
            position: 1,
            position_goal_end: 2,
            lost: false,
            ..Default::default()
        };

        let new_state = update(current_state, 0);
        assert_eq!(false, new_state.lost);
    }

    #[test]
    fn game_is_won_if_speed_is_0_and_position_between_lower_and_upper_goal_bounds() {
        let current_state = State {
            position_goal_start: 1,
            position_goal_end: 3,
            position: 2,
            speed: 0,
            won: false,
            ..Default::default()
        };

        let new_state = update(current_state, 0);
        assert_eq!(true, new_state.won);
    }

    #[test]
    fn game_should_keep_goal_start_and_end_values_on_update(){
        let current_state = State {
            position_goal_end: 2,
            position_goal_start: 1,
            ..Default::default()
        };

        let new_state = update(current_state, 0);
        assert_eq!(1, new_state.position_goal_start);
        assert_eq!(2, new_state.position_goal_end);
    }

    #[test]
    fn throttle_should_not_directly_update_speed_or_position(){
        let current_state = State {
            position: 0,
            speed: 0,
            ..Default::default()
        };

        let new_state = update(current_state, 1);
        assert_eq!(0, new_state.position);
        assert_eq!(0, new_state.speed);
    }

    #[test]
    fn negative_acceleration_should_not_make_speed_negative(){
        let current_state = State {
            acceleration: -1,
            speed: 0,
            ..Default::default()
        };

        let new_state = update(current_state, 0);
        assert_eq!(0, new_state.speed);
    }


    #[test]
    fn camera_should_project_900_when_car_world_position_is_100_and_camera_size_is_1000_and_world_and_screen_size_are_the_same(){
        let camera = Camera {
            screen_size: 1000,
            world_size: 1000,
            world_position: 500,
        };
        let screen_position = camera.project(100);
        assert_eq!(900, screen_position);
    }

    #[test]
    fn camera_should_project_900_when_world_position_is_1000_and_camera_world_size_is_10x_bigger_than_screen_size(){
        let camera= Camera {
            screen_size: 1000,
            world_size: 10000,
            world_position: 5000,
        };
        let screen_position = camera.project(1000);
        assert_eq!(900, screen_position);
    }

    #[test]
    fn camera_should_project_900_when_camera_is_at_5000_in_world_position(){
        let camera = Camera {
            screen_size: 1000,
            world_size: 10000,
            world_position: 5000
        };
        let screen_position = camera.project(1000);
        assert_eq!(900, screen_position);
    }

    #[test]
    fn camera_should_project_500_when_camera_is_at_same_world_position_than_target(){
        let camera = Camera {
            screen_size: 1000,
            world_size: 10000,
            world_position: 1000
        };
        let screen_position = camera.project(1000);
        assert_eq!(500, screen_position);
    }
}
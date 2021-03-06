use legion::prelude::*;

use crate::resources::InputResource;

// Call this to process input state
pub fn update_input_resource() -> Box<dyn Schedulable> {
    SystemBuilder::new("input end frame")
        .write_resource::<InputResource>()
        .build(|_, _, _input, _| {
            //input.end_frame();
        })
}

// Call this to mark the start of the next frame (i.e. "key just down" will return false). This goes
// at the end of the frame, winit will fire events after we exit the frame, and then
// update_input_resource will be called at the start of the next frame
pub fn input_reset_for_next_frame() -> Box<dyn Schedulable> {
    SystemBuilder::new("input end frame")
        .write_resource::<InputResource>()
        .build(|_, _, input, _| {
            input.end_frame();
        })
}

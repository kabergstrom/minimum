use minimum::resource::{DataRequirement, Write};
use minimum::{Task, TaskContext};
use named_type::NamedType;

use crate::resources::DebugDraw;

#[derive(NamedType)]
pub struct ClearDebugDraw;
impl Task for ClearDebugDraw {
    type RequiredResources = (Write<DebugDraw>);
    const REQUIRED_FLAGS: usize = crate::context_flags::AUTHORITY_CLIENT as usize;

    fn run(
        &mut self,
        _task_context: &TaskContext,
        data: <Self::RequiredResources as DataRequirement>::Borrow,
    ) {
        let mut debug_draw = data;

        debug_draw.clear();
    }
}
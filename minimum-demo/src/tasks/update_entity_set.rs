
use minimum::task::ReadAllTaskImpl;
use minimum::TaskConfig;
use minimum::ResourceMap;
use minimum::EntitySet;


pub struct UpdateEntitySet;
pub type UpdateEntitySetTask = minimum::ReadAllTask<UpdateEntitySet>;
impl ReadAllTaskImpl for UpdateEntitySet {
    fn configure(config: &mut TaskConfig) {
        config.this_runs_during_phase::<minimum::task::PhaseEndFrame>();
    }

    fn run(resource_map: &ResourceMap) {
        let mut entity_set = resource_map.fetch_mut::<EntitySet>();
        entity_set.update(resource_map);
    }
}

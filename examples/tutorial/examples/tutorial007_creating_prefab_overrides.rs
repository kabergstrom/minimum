use legion::prelude::*;
use legion::world::{NoneCloneImplResult, NoneEntityReplacePolicy};
use glam::Vec2;

use type_uuid::TypeUuid;

use serde::Serialize;
use serde::Deserialize;
use serde_diff::SerdeDiff;
use std::collections::HashMap;
use legion_prefab::{SpawnCloneImpl, Prefab, ComponentRegistration, PrefabBuilder};
use uuid::adapter::compact::serialize;
use minimum::ComponentRegistry;
use prefab_format::EntityUuid;

#[derive(TypeUuid, Clone, Serialize, Deserialize, SerdeDiff, Debug, Default)]
#[uuid = "8bf67228-f96c-4649-b306-ecd107190000"]
pub struct PositionComponent {
    #[serde_diff(opaque)]
    pub value: Vec2,
}

legion_prefab::register_component_type!(PositionComponent);

#[derive(TypeUuid, Clone, Serialize, Deserialize, SerdeDiff, Debug, Default)]
#[uuid = "8bf67228-f96c-4649-b306-ecd107190001"]
pub struct VelocityComponent {
    #[serde_diff(opaque)]
    pub value: Vec2,
}

legion_prefab::register_component_type!(VelocityComponent);

fn main() {
    // Setup logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // Spawn the daemon in a background thread. This could be a different process, but
    // for simplicity we'll launch it here.
    std::thread::spawn(move || {
        minimum::daemon::run();
    });

    // Register all components (based on legion_prefab::register_component_type! macro)
    let component_registry = minimum::ComponentRegistryBuilder::new()
        .auto_register_components()
        .build();

    // Create a legion universe
    let universe = Universe::new();

    // Create a world and insert data into it that we would like to save into a prefab
    let mut prefab_world = universe.create_world();
    let prefab_entity = *prefab_world
        .insert(
            (),
            (0..1).map(|_| {
                (
                    PositionComponent {
                        value: Vec2::new(0.0, 500.0),
                    },
                    VelocityComponent {
                        value: Vec2::new(5.0, 0.0),
                    },
                )
            }),
        )
        .first()
        .unwrap();

    // Create the prefab
    let prefab = Prefab::new(prefab_world);

    // Get the UUID of the entity. This UUID is maintained throughout the whole chain.
    let entity_uuid = prefab
        .prefab_meta
        .entities
        .iter()
        .find(|(_, value)| **value == prefab_entity)
        .map(|(entity_uuid, _)| *entity_uuid)
        .unwrap();

    //
    // Cook the prefab world we just deserialized
    //
    let prefab_cook_order = vec![prefab.prefab_id()];
    let mut prefab_lookup = HashMap::new();
    prefab_lookup.insert(prefab.prefab_id(), &prefab);

    let cooked_prefab = legion_prefab::cook_prefab(
        &universe,
        component_registry.components(),
        component_registry.components_by_uuid(),
        prefab_cook_order.as_slice(),
        &prefab_lookup,
    );

    //
    // Use a prefab builder to make a new prefab that overrides a field on the given prefab
    //
    let mut prefab_builder = PrefabBuilder::new(
        prefab.prefab_id(),
        cooked_prefab,
        &universe,
        &component_registry.copy_clone_impl(),
    );
    let prefab_builder_entity = prefab_builder.uuid_to_entity(entity_uuid).unwrap();
    prefab_builder
        .world_mut()
        .get_component_mut::<PositionComponent>(prefab_builder_entity)
        .unwrap()
        .value = glam::Vec2::new(0.0, 1000.0);

    let prefab_with_override = prefab_builder
        .create_prefab(
            &universe,
            component_registry.components_by_uuid(),
            &component_registry.copy_clone_impl(),
        )
        .unwrap();

    //
    // Cook the prefab that has the override
    //
    let prefab_cook_order = vec![prefab.prefab_id(), prefab_with_override.prefab_id()];
    let mut prefab_lookup = HashMap::new();
    prefab_lookup.insert(prefab.prefab_id(), &prefab);
    prefab_lookup.insert(prefab_with_override.prefab_id(), &prefab_with_override);

    let cooked_prefab_with_override = legion_prefab::cook_prefab(
        &universe,
        component_registry.components(),
        component_registry.components_by_uuid(),
        prefab_cook_order.as_slice(),
        &prefab_lookup,
    );

    // Look up the entity in the cooked prefab with override by UUID
    let entity = cooked_prefab_with_override.entities[&entity_uuid];

    let position = cooked_prefab_with_override
        .world
        .get_component::<PositionComponent>(entity)
        .unwrap();
    println!(
        "Position of {} is {}",
        uuid::Uuid::from_bytes(entity_uuid),
        position.value
    );
}

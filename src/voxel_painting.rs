use crate::{
    core_components::{PaintableResources, PlayerWantsToPaintVoxel},
    map_setup::*,
    spawner::spawn_organism,
};
use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

const STEP_SIZE: f32 = 0.99;
const MAX_SEARCH_DISTANCE: f32 = 518.; // ceil(512/0.99)

pub fn ray_cast_to_voxel(
    voxel_world: &VoxelWorld,
    player_position: Vec3,
    look_direction: Vec3,
) -> Option<(IVec3, WorldVoxel)> {
    let step = STEP_SIZE * look_direction.normalize();

    (0..)
        .map(|ii| player_position + step * (ii as f32))
        .take_while(|&pos| pos.distance(player_position) <= MAX_SEARCH_DISTANCE)
        .map(|vec| vec.as_ivec3())
        .map(|voxel_pos| (voxel_pos, voxel_world.get_voxel(voxel_pos)))
        .find(|&(_, voxel)| matches!(voxel, WorldVoxel::Solid(_)))
}

pub fn get_surface_air_voxel(
    voxel_world: &mut VoxelWorld,
    player_position: Vec3,
    look_direction: Vec3,
) -> Option<IVec3> {
    if let Some((vox_pos, voxel)) = ray_cast_to_voxel(voxel_world, player_position, look_direction)
    {
        let above_pos = vox_pos + IVec3::Y;
        let above_vox = voxel_world.get_voxel(above_pos);
        match (voxel, above_vox) {
            (WorldVoxel::Solid(DIRT_BLOCK), WorldVoxel::Air) => Some(above_pos),
            _ => None,
        }
    } else {
        None
    }
}

pub fn paint_voxel_system(
    mut commands: Commands,
    mut voxel_world: VoxelWorld,
    paint_query: Query<(Entity, &PlayerWantsToPaintVoxel)>,
) {
    paint_query.for_each(|(paint_entity, want_to_paint)| {
        // TODO: maybe add more checks to see if it is OK to paint,
        // but probably don't want to be redundant with get_surface_air_voxel
        let PaintableResources::SeedCrop(species) = want_to_paint.paint_as.clone();
        spawn_organism(
            &mut commands,
            species,
            crate::core_components::LifePhase::Seed,
            want_to_paint.pos,
            Some(want_to_paint.player),
        );
        // FIXME: rework to not be hardcoded  once block types are an enum
        voxel_world.set_voxel(want_to_paint.pos, WorldVoxel::Solid(WHEAT_BLOCK));
        commands.entity(paint_entity).despawn();
    })
}

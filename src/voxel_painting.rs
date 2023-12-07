use crate::{block_types::BlockType, core_components::*, spawner::spawn_organism};
use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

const STEP_SIZE: f32 = 0.99;
const MAX_SEARCH_DISTANCE: f32 = 518.; // ceil(512/STEP_SIZE)

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
        let vox_pair = (voxel, above_vox);
        if vox_pair == (WorldVoxel::Solid(BlockType::Dirt.index()), WorldVoxel::Air) {
            Some(above_pos)
        } else {
            None
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
            species.clone(),
            crate::core_components::LifePhase::Seed,
            want_to_paint.pos,
            Some(want_to_paint.player),
        );

        let block_type = species.block_type(&SEED_PHASE);
        paint_voxel_unchecked(&mut voxel_world, want_to_paint.pos, block_type);
        commands.entity(paint_entity).despawn();
    })
}

/// It is assumed that the type of voxel to be painted is correct according
/// to the game logic, as `paint_voxel_unchecked` does no checking
pub fn paint_voxel_unchecked(voxel_world: &mut VoxelWorld, pos: IVec3, block_type: BlockType) {
    voxel_world.set_voxel(pos, WorldVoxel::Solid(block_type.index()));
}

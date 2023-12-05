use crate::map_setup::*;
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

pub fn paint_on_voxel(
    voxel_world: &mut VoxelWorld,
    player_position: Vec3,
    look_direction: Vec3,
) -> Option<IVec3> {
    if let Some((vox_pos, voxel)) = ray_cast_to_voxel(voxel_world, player_position, look_direction)
    {
        let above_pos = vox_pos + IVec3::Y;
        let above_vox = voxel_world.get_voxel(above_pos);
        match (voxel, above_vox) {
            (WorldVoxel::Solid(DIRT_BLOCK), WorldVoxel::Air) => {
                voxel_world.set_voxel(above_pos, WorldVoxel::Solid(WHEAT_BLOCK));
                Some(above_pos)
            }
            _ => None,
        }
    } else {
        None
    }
}

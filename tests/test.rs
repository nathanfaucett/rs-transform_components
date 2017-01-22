extern crate ecs;
extern crate atomic;
extern crate transform_components;


use ecs::Scene;
use transform_components::{Transform2D, Transform3D, TransformProcess};


static SIZE: usize = 1024usize;


#[test]
fn test_transform2d() {
    let mut scene = Scene::new();

    {
        let mut entity_manager = scene.entity_manager_mut().unwrap();

        for _ in 0..SIZE {
            let entity = entity_manager.new_entity();
            let mut transform = Transform2D::new(entity);
            transform.set_position(&[1f32; 2]);
            entity_manager.add_component(&entity, transform);
        }
    }

    scene.process_manager_mut().add_process(TransformProcess::new());

    scene.update();

    for (_entity, transform2d) in scene.entity_manager().unwrap().component_iter::<Transform2D>() {
        assert_eq!(transform2d.local_matrix(), &[1f32, 0f32, -0f32, 1f32, 1f32, 1f32]);
    }
}
#[test]
fn test_transform3d() {
    let mut scene = Scene::new();

    {
        let mut entity_manager = scene.entity_manager_mut().unwrap();

        for _ in 0..SIZE {
            let entity = entity_manager.new_entity();
            let mut transform = Transform3D::new(entity);
            transform.set_position(&[1f32; 3]);
            entity_manager.add_component(&entity, transform);
        }
    }

    scene.process_manager_mut().add_process(TransformProcess::new());

    scene.update();

    for (_entity, transform3d) in scene.entity_manager().unwrap().component_iter::<Transform3D>() {
        assert_eq!(transform3d.local_matrix(), &[
            1f32, 0f32, 0f32, 0f32,
            0f32, 1f32, 0f32, 0f32,
            0f32, 0f32, 1f32, 0f32,
            1f32, 1f32, 1f32, 1f32
        ]);
    }
}

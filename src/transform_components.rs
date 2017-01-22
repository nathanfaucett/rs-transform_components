use ecs::{Entity, Process, EntityManager, Hierarchy};
use atomic::AtomicValue;

use vec2;
use vec3;
use quat;
use mat32;
use mat3;
use mat4;


#[derive(Debug, PartialEq)]
pub struct Transform2D {
    entity: Entity,

    pub local_position: [f32; 2],
    pub local_scale: [f32; 2],
    pub local_rotation: f32,

    position: [f32; 2],
    scale: [f32; 2],
    rotation: f32,

    needs_update: bool,
    local_matrix: [f32; 6],
    matrix: [f32; 6],
}

impl Transform2D {
    pub fn new(entity: Entity) -> Self {
        Transform2D {
            entity: entity,

            local_position: [0f32; 2],
            local_scale: [1f32; 2],
            local_rotation: 0f32,

            position: [0f32; 2],
            scale: [1f32; 2],
            rotation: 0f32,

            needs_update: false,
            local_matrix: mat32::new_identity(),
            matrix: mat32::new_identity(),
        }
    }
    pub fn needs_update(&self) -> bool {
        self.needs_update
    }
    pub fn set_needs_update(&mut self) {
        self.needs_update = true;
    }

    pub fn position(&self) -> &[f32; 2] { &self.position }
    pub fn local_position(&self) -> &[f32; 2] { &self.local_position }
    pub fn local_position_mut(&mut self) -> &mut [f32; 2] { &mut self.local_position }

    pub fn set_position(&mut self, position: &[f32; 2]) -> &mut Self {
        vec2::copy(&mut self.local_position, position);
        self.set_needs_update();
        self
    }

    pub fn scale(&self) -> &[f32; 2] { &self.scale }
    pub fn local_scale(&self) -> &[f32; 2] { &self.local_scale }
    pub fn local_scale_mut(&mut self) -> &mut [f32; 2] { &mut self.local_scale }

    pub fn set_scale(&mut self, scale: &[f32; 2]) -> &mut Self {
        vec2::copy(&mut self.local_scale, scale);
        self.set_needs_update();
        self
    }

    pub fn rotation(&self) -> f32 { self.rotation }
    pub fn local_rotation(&self) -> f32 { self.local_rotation }
    pub fn local_rotation_mut(&mut self) -> &mut f32 { &mut self.local_rotation }

    pub fn set_rotation(&mut self, rotation: f32) -> &mut Self {
        self.local_rotation = rotation;
        self.set_needs_update();
        self
    }

    pub fn translate(&mut self, translation: &[f32; 2]) -> &mut Self {
        let local_position = self.local_position;
        vec2::add(&mut self.local_position, &local_position, translation);
        self.set_needs_update();
        self
    }
    pub fn rotate(&mut self, rotation: f32) -> &mut Self {
        self.local_rotation = self.local_rotation + rotation;
        self.set_needs_update();
        self
    }

    pub fn look_at(&mut self, target: &[f32; 2]) -> &mut Self {
        {
            let mut mat = mat32::new_identity();

            mat32::look_at(&mut mat, self.position(), target);
            self.local_rotation = mat32::get_rotation(&mat);

            self.set_needs_update();
        }
        self
    }

    pub fn local_to_world<'a, 'b>(&self, out: &'a mut [f32; 2], v: &'b [f32; 2]) -> &'a mut [f32; 2] {
        vec2::transform_mat32(out, &v, &self.matrix())
    }

    pub fn world_to_local<'a, 'b>(&self, out: &'a mut [f32; 2], v: &'b [f32; 2]) -> &'a mut [f32; 2] {
        let mut m = mat32::new_identity();
        mat32::inverse(&mut m, &self.matrix());
        vec2::transform_mat32(out, &v, &m)
    }

    pub fn world_matrix(&self) -> [f32; 16] {
        let matrix = self.matrix();
        let mut m = mat4::new_identity();
        m[0] = matrix[0];
        m[4] = matrix[2];
        m[1] = matrix[1];
        m[5] = matrix[3];
        m[12] = matrix[4];
        m[13] = matrix[5];
        m
    }
    pub fn model_view_matrix<'a, 'b>(
        &self, model_view_matrix: &'a mut [f32; 16], view_matrix: &'b [f32; 16]
    ) -> &'a mut [f32; 16] {
        mat4::mul(model_view_matrix, view_matrix, &self.world_matrix())
    }
    pub fn normal_matrix<'a, 'b>(
        &self, normal_matrix: &'a mut [f32; 9], model_view_matrix: &'b [f32; 16]
    ) -> &'a mut [f32; 9] {
        let mut n = mat3::new_identity();
        mat3::inverse_mat4(&mut n, model_view_matrix);
        mat3::transpose(normal_matrix, &n)
    }

    pub fn matrix(&self) -> &[f32; 6] { &self.matrix }
    pub fn local_matrix(&self) -> &[f32; 6] { &self.local_matrix }
}

#[derive(Debug, PartialEq)]
pub struct Transform3D {
    entity: Entity,

    pub local_position: [f32; 3],
    pub local_scale: [f32; 3],
    pub local_rotation: [f32; 4],

    position: [f32; 3],
    scale: [f32; 3],
    rotation: [f32; 4],

    needs_update: bool,
    local_matrix: [f32; 16],
    matrix: [f32; 16],
}

impl Transform3D {
    pub fn new(entity: Entity) -> Self {
        Transform3D {
            entity: entity,

            local_position: [0f32; 3],
            local_scale: [1f32; 3],
            local_rotation: [0f32, 0f32, 0f32, 1f32],

            position: [0f32; 3],
            scale: [1f32; 3],
            rotation: [0f32, 0f32, 0f32, 1f32],

            needs_update: false,
            local_matrix: mat4::new_identity(),
            matrix: mat4::new_identity(),
        }
    }
    pub fn needs_update(&self) -> bool {
        self.needs_update
    }
    pub fn set_needs_update(&mut self) {
        self.needs_update = true;
    }

    pub fn position(&self) -> &[f32; 3] { &self.position }
    pub fn local_position(&self) -> &[f32; 3] { &self.local_position }
    pub fn local_position_mut(&mut self) -> &mut [f32; 3] { &mut self.local_position }

    pub fn set_position(&mut self, position: &[f32; 3]) -> &mut Self {
        vec3::copy(&mut self.local_position, position);
        self.set_needs_update();
        self
    }

    pub fn scale(&self) -> &[f32; 3] { &self.scale }
    pub fn local_scale(&self) -> &[f32; 3] { &self.local_scale }
    pub fn local_scale_mut(&mut self) -> &mut [f32; 3] { &mut self.local_scale }

    pub fn set_scale(&mut self, scale: &[f32; 3]) -> &mut Self {
        vec3::copy(&mut self.local_scale, scale);
        self.set_needs_update();
        self
    }

    pub fn rotation(&self) -> &[f32; 4] { &self.rotation }
    pub fn local_rotation(&self) -> &[f32; 4] { &self.local_rotation }
    pub fn local_rotation_mut(&mut self) -> &mut [f32; 4] { &mut self.local_rotation }

    pub fn set_rotation(&mut self, rotation: &[f32; 4]) -> &mut Self {
        quat::copy(&mut self.local_rotation, rotation);
        self.set_needs_update();
        self
    }

    pub fn translate(&mut self, translation: &[f32; 3]) -> &mut Self {
        let local_position = self.local_position;
        vec3::add(&mut self.local_position, &local_position, translation);
        self.set_needs_update();
        self
    }
    pub fn rotate(&mut self, rotation: &[f32; 3]) -> &mut Self {
        let local_rotation = self.local_rotation;
        quat::rotate(&mut self.local_rotation, &local_rotation, rotation[0], rotation[1], rotation[2]);
        self.set_needs_update();
        self
    }

    pub fn look_at(&mut self, target: &[f32; 3], up: &[f32; 3]) -> &mut Self {
        let mut mat = mat4::new_identity();

        mat4::look_at(&mut mat, self.position(), target, up);
        quat::from_mat4(&mut self.local_rotation, &mat);

        self.set_needs_update();
        self
    }

    pub fn local_to_world<'a, 'b>(&mut self, out: &'a mut [f32; 3], v: &'b [f32; 3]) -> &'a mut [f32; 3] {
        vec3::transform_mat4(out, &v, &self.matrix())
    }

    pub fn world_to_local<'a, 'b>(&mut self, out: &'a mut [f32; 3], v: &'b [f32; 3]) -> &'a mut [f32; 3] {
        let mut m = mat4::new_identity();
        mat4::inverse(&mut m, &self.matrix());
        vec3::transform_mat4(out, &v, &m)
    }

    pub fn world_matrix(&self) -> &[f32; 16] {
        self.matrix()
    }
    pub fn model_view_matrix<'a, 'b>(
        &self, model_view_matrix: &'a mut [f32; 16], view_matrix: &'b [f32; 16]
    ) -> &'a mut [f32; 16] {
        mat4::mul(model_view_matrix, view_matrix, &self.world_matrix())
    }
    pub fn normal_matrix<'a, 'b>(
        &self, normal_matrix: &'a mut [f32; 9], model_view_matrix: &'b [f32; 16]
    ) -> &'a mut [f32; 9] {
        let mut n = mat3::new_identity();
        mat3::inverse_mat4(&mut n, model_view_matrix);
        mat3::transpose(normal_matrix, &n)
    }

    pub fn matrix(&self) -> &[f32; 16] { &self.matrix }
    pub fn local_matrix(&self) -> &[f32; 16] { &self.local_matrix }
}


#[derive(Debug, Eq, PartialEq)]
pub struct TransformProcess;

impl TransformProcess {
    #[inline(always)]
    pub fn new() -> Self { TransformProcess }

    fn update_matrix_transform2d(entity_manager: &EntityManager, transform2d: &AtomicValue<Transform2D>) {
        let mut transform2d_lock = transform2d.as_mut().unwrap();

        if transform2d_lock.needs_update {
            transform2d_lock.needs_update = false;

            { // update local matrix
                let local_position = transform2d.local_position;
                let local_scale = transform2d.local_scale;
                let local_rotation = transform2d.local_rotation;
                mat32::compose(
                    &mut transform2d_lock.local_matrix,
                    &local_position,
                    &local_scale,
                    local_rotation
                );
            }
            { // update world matrix relative to parent
                let mut has_parent = false;

                if let Some(hierarchy) = entity_manager.component::<Hierarchy>(&transform2d.entity) {
                    if let Some(parent) = hierarchy.parent() {
                        if let Some(ref parent_transform) = entity_manager.component::<Transform2D>(&parent) {
                            Self::update_matrix_transform2d(entity_manager, parent_transform);
                            let parent_transform_lock = parent_transform.as_mut().unwrap();

                            let parent_matrix = parent_transform_lock.matrix();
                            let local_matrix = transform2d_lock.local_matrix;
                            mat32::mul(
                                &mut transform2d_lock.matrix,
                                &parent_matrix,
                                &local_matrix
                            );
                            has_parent = true;
                        } else if let Some(ref parent_transform) = entity_manager.component::<Transform3D>(&parent) {
                            Self::update_matrix_transform3d(entity_manager, parent_transform);
                            let parent_transform_lock = parent_transform.as_mut().unwrap();

                            let mut parent_matrix = mat32::new_identity();
                            let local_matrix = transform2d_lock.local_matrix;
                            mat32::from_mat4(
                                &mut parent_matrix,
                                parent_transform_lock.matrix()
                            );
                            mat32::mul(
                                &mut transform2d_lock.matrix,
                                &parent_matrix,
                                &local_matrix
                            );
                            has_parent = true;
                        }
                    }
                }

                if !has_parent {
                    let local_matrix = transform2d_lock.local_matrix;
                    mat32::copy(&mut transform2d_lock.matrix, &local_matrix);
                }
            }
            {// update world position scale and rotation
                let matrix = transform2d_lock.matrix;
                let mut position = transform2d_lock.position;
                let mut scale = transform2d_lock.scale;
                transform2d_lock.rotation = mat32::decompose(
                    &matrix,
                    &mut position,
                    &mut scale
                );
                transform2d_lock.position = position;
                transform2d_lock.scale = scale;
            }
        }
    }
    fn update_matrix_transform3d(entity_manager: &EntityManager, transform3d: &AtomicValue<Transform3D>) {
        if transform3d.needs_update {
            let mut transform3d_lock = transform3d.as_mut().unwrap();

            transform3d_lock.needs_update = false;

            // update local matrix
            mat4::compose(
                &mut transform3d_lock.local_matrix,
                &transform3d.local_position,
                &transform3d.local_scale,
                &transform3d.local_rotation
            );
            { // update world matrix relative to parent
                let mut has_parent = false;

                if let Some(hierarchy) = entity_manager.component::<Hierarchy>(&transform3d.entity) {
                    if let Some(parent) = hierarchy.parent() {
                        if let Some(ref parent_transform) = entity_manager.component::<Transform3D>(&parent) {
                            Self::update_matrix_transform3d(entity_manager, parent_transform);
                            let parent_transform_lock = parent_transform.as_mut().unwrap();

                            let parent_matrix = parent_transform_lock.matrix();
                            let local_matrix = transform3d_lock.local_matrix;
                            mat4::mul(
                                &mut transform3d_lock.matrix,
                                &parent_matrix,
                                &local_matrix
                            );
                            has_parent = true;
                        } else if let Some(ref parent_transform) = entity_manager.component::<Transform2D>(&parent) {
                            Self::update_matrix_transform2d(entity_manager, parent_transform);
                            let parent_transform_lock = parent_transform.as_mut().unwrap();

                            let parent_matrix = parent_transform_lock.world_matrix();
                            let local_matrix = transform3d_lock.local_matrix;
                            mat4::mul(
                                &mut transform3d_lock.matrix,
                                &parent_matrix,
                                &local_matrix
                            );
                            has_parent = true;
                        }
                    }
                }

                if !has_parent {
                    let local_matrix = transform3d_lock.local_matrix;
                    mat4::copy(&mut transform3d_lock.matrix, &local_matrix);
                }
            }
            {// update world position scale and rotation
                let matrix = transform3d_lock.matrix;
                let mut position = transform3d_lock.position;
                let mut scale = transform3d_lock.scale;
                let mut rotation = transform3d_lock.rotation;
                mat4::decompose(
                    &matrix,
                    &mut position,
                    &mut scale,
                    &mut rotation
                );
                transform3d_lock.position = position;
                transform3d_lock.scale = scale;
                transform3d_lock.rotation = rotation;
            }
        }
    }
}

impl Process for TransformProcess {
    fn handle(&mut self, entity_manager: &EntityManager) {
        for (_entity, transform2d) in entity_manager.component_iter::<Transform2D>() {
            Self::update_matrix_transform2d(entity_manager, &transform2d);
        }
        for (_entity, transform3d) in entity_manager.component_iter::<Transform3D>() {
            Self::update_matrix_transform3d(entity_manager, &transform3d);
        }
    }
}

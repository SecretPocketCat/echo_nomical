use bevy::{
    ecs::system::{Command, EntityCommands},
    prelude::*,
};

pub struct TryInsert<T> {
    pub entity: Entity,
    pub bundle: T,
}

impl<T> Command for TryInsert<T>
where
    T: Bundle + 'static,
{
    fn write(self, world: &mut World) {
        if let Some(mut entity) = world.get_entity_mut(self.entity) {
            entity.insert(self.bundle);
        }
    }
}

pub trait EntityCommandsExt {
    fn try_insert(&mut self, bundle: impl Bundle) -> &mut Self;
}

impl<'w, 's, 'a> EntityCommandsExt for EntityCommands<'w, 's, 'a> {
    fn try_insert(&mut self, bundle: impl Bundle) -> &mut Self {
        let e = self.id();
        self.commands().add(TryInsert { entity: e, bundle });
        self
    }
}

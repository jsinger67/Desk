use bevy_ecs::schedule::SystemLabel;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum DeskSystem {
    Update,
    RenderWidget,
    PrePhysics,
}

#[derive(SystemLabel, PartialEq, Eq, Debug, Hash, Clone)]
pub enum ShellSystem {
    Add,
    UpdateComponent,
    UpdateWidget,
    Render,
    HandleEvents,
}

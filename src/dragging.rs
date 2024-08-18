use crate::prelude::*;

pub struct DraggingPlugin;

impl Plugin for DraggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DragServerEvent>()
            .add_event::<DragServerStartEvent>()
            .add_event::<DragServerEndEvent>()
            .add_event::<DragOutputEvent>()
            .add_event::<DragOutputStartEvent>()
            .add_event::<DragOutputEndEvent>()
            .add_systems(
                Update,
                (handle_dragging_server, handle_drag_start, handle_drag_end)
                    .run_if(in_state(GameState::Planning)),
            );
    }
}

#[derive(Event)]
pub struct IsBeingDragged;

macro_rules! define_drag_event {
    ($struct_name:ident, $pointer_type:ty, $entity:ty, $vec:ty) => {
        #[derive(Event)]
        pub struct $struct_name(pub $entity, pub $vec);

        impl From<ListenerInput<Pointer<$pointer_type>>> for $struct_name {
            fn from(event: ListenerInput<Pointer<$pointer_type>>) -> Self {
                $struct_name(event.target, event.delta)
            }
        }
    };
    ($struct_name:ident, $pointer_type:ty, $entity:ty) => {
        #[derive(Event)]
        pub struct $struct_name(pub $entity);

        impl From<ListenerInput<Pointer<$pointer_type>>> for $struct_name {
            fn from(event: ListenerInput<Pointer<$pointer_type>>) -> Self {
                $struct_name(event.target)
            }
        }
    };
}

// Servers
define_drag_event!(DragServerEvent, Drag, Entity, Vec2);
define_drag_event!(DragServerStartEvent, DragStart, Entity);
define_drag_event!(DragServerEndEvent, DragEnd, Entity);

// Server Output Dragging
define_drag_event!(DragOutputEvent, Drag, Entity, Vec2);
define_drag_event!(DragOutputStartEvent, DragStart, Entity);
define_drag_event!(DragOutputEndEvent, DragEnd, Entity);

// #[derive(Event)]
// pub struct DragServerEvent(pub Entity, pub Vec2);

// impl From<ListenerInput<Pointer<Drag>>> for DragServerEvent {
//     fn from(event: ListenerInput<Pointer<Drag>>) -> Self {
//         DragServerEvent(event.target, event.delta)
//     }
// }

// #[derive(Event)]
// pub struct DragServerStartEvent(pub Entity);

// impl From<ListenerInput<Pointer<DragStart>>> for DragServerStartEvent {
//     fn from(event: ListenerInput<Pointer<DragStart>>) -> Self {
//         DragServerStartEvent(event.target)
//     }
// }

// #[derive(Event)]
// pub struct DragServerEndEvent(pub Entity);

// impl From<ListenerInput<Pointer<DragEnd>>> for DragServerEndEvent {
//     fn from(event: ListenerInput<Pointer<DragEnd>>) -> Self {
//         DragServerEndEvent(event.target)
//     }
// }

fn handle_dragging_server(
    mut events_drag: EventReader<DragServerEvent>,
    mut q_transform: Query<&mut Transform, With<Server>>,
) {
    for ev in events_drag.read() {
        if let Ok(mut transform) = q_transform.get_mut(ev.0) {
            transform.translation.y -= ev.1.y;
            transform.translation.x += ev.1.x;
            // let new_vec2 = transform.translation.truncate() + inverted_drag_vector;
            // let new_vec3 = Vec3::new(new_vec2.x, new_vec2.y, transform.translation.z);
            // let inverted_drag_vector = Vec2::new(ev.1.x, -ev.1.y);
            // transform.translation = new_vec3;
        } else {
            // We're dragging something else?
        }
    }
}

fn handle_drag_start(
    mut events_drag_start: EventReader<DragServerStartEvent>,
    mut commands: Commands,
) {
    for ev in events_drag_start.read() {
        commands.entity(ev.0).insert(IsBeingDragged);
        println!("Drag start");
    }
}

fn handle_drag_end(
    mut events_drag_end: EventReader<DragServerEndEvent>,
    mut commands: Commands,
    mut evs: EventWriter<AlignServersEvent>,
) {
    for ev in events_drag_end.read() {
        println!("Drag end");
        commands.entity(ev.0).remove::<IsBeingDragged>();
        evs.send(AlignServersEvent);
    }
}

use crate::prelude::*;

use bevy_mod_picking::{debug::DebugPickingMode, prelude::PickSelection, DefaultPickingPlugins};

/*

Add components:

PickableBundle::default(),
On::<Pointer<Click>>::send_event::<SelectEvent>(),

 */

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins);
        // .insert_resource(DebugPickingMode::Normal);
        // .insert_resource(Selection::default())
        // .add_systems(Update, print_selected);
        // app.add_event::<UpgradeServerCPUEvent>()
        //     .add_event::<AddNewServer>()
        //     .add_systems(
        //         Update,
        //         (
        //             handle_upgrade_server_cpu,
        //             draw_children_ui,
        //             add_new_server.run_if(on_event::<AddNewServer>()),
        //             align_servers_to_grid,
        //         ),
        //     )
        //     .add_systems(FixedUpdate, process_requests);
    }
}

// #[derive(Default)]
// pub enum SelectedType {
//     #[default]
//     Server,
//     LoadBalancer,
// }

// #[derive(Resource, Default)]
// pub struct Selection {
//     pub selected: Option<Entity>,
// }

// #[derive(Event)]
// pub struct SelectEvent(pub Entity);

// impl From<ListenerInput<Pointer<Click>>> for SelectEvent {
//     fn from(event: ListenerInput<Pointer<Click>>) -> Self {
//         SelectEvent(event.target)
//     }
// }

// pub fn handle_select_event(
//     mut res: ResMut<Selection>,
//     mut evs: EventReader<SelectEvent>,
//     q_server: Query<&Server>,
// ) {
//     for ev in evs.read() {
//         println!("Received click event");
//         res.selected = Some(ev.0);
//     }
// }

pub fn draw_selected(
    mut gizmos: Gizmos,
    selected: Query<(Entity, &PickSelection, &Transform, &Server)>,
) {
    for (_entity, pick_selection, transform, _server) in selected.iter() {
        if pick_selection.is_selected {
            let color = Color::srgba(0.73333335, 0.96862745, 0.8156863, 0.3);
            gizmos
                .rounded_rect_2d(
                    transform.translation.truncate(),
                    0.,
                    Vec2::splat(80.0),
                    color,
                )
                .corner_radius(0.5)
                .arc_resolution(10);
        }
    }
}

// pub fn handle_deselect(mut evs: EventReader<MouseButtonInput>, mut selection: ResMut<Selection>) {
//     for ev in evs.read() {
//         selection.selected = None;
//     }
// }

// fn print_selected(query: Query<&PickSelection>) {
//     for i in query.iter() {
//         if i.is_selected {
//             println!("Selected: {:#?}", i);
//         }
//     }
// }

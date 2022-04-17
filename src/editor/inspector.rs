use bevy::prelude::*;
use bevy_inspector_egui::{egui, Inspectable};
use serde::{Deserialize, Serialize};

use crate::{
    controller::layout::ControllerKey,
    input::listener::{InputListener, ListenerResult},
};

// Data to identify a specific `InputSource` belonging to an `InputSink` component.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct InputSinkId {
    // The entity containing the `InputSink` component.
    pub entity: Option<Entity>,
    // The index of the identified `InputSource` in the sink's `sources` vector.
    pub idx: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundControllerKey {
    #[serde(skip)]
    pub id: Option<InputSinkId>,
    pub key: Option<ControllerKey>,
}

impl BoundControllerKey {
    pub fn bind(&mut self, entity: Entity, idx: usize) {
        self.id = Some(InputSinkId {
            entity: Some(entity),
            idx,
        });
    }
}

impl From<ControllerKey> for BoundControllerKey {
    fn from(key: ControllerKey) -> Self {
        BoundControllerKey {
            id: None,
            key: Some(key),
        }
    }
}

impl Inspectable for BoundControllerKey {
    type Attributes = ();

    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _: Self::Attributes,
        context: &mut bevy_inspector_egui::Context,
    ) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            // Render the currently bound `ControllerKey` value as a label.
            let label_text = match self.key {
                Some(key) => key.to_string(),
                None => "-".to_string(),
            };
            ui.label(label_text);

            // Render a button to rebind this key.
            context.resource_scope(
                ui,
                "InputListener",
                |mut ui, ctx, mut input_listener: Mut<InputListener>| {
                    if let Some(sink_id) = self.id {
                        if input_listener.has_sink_consumer(sink_id) {
                            ui.button("listening...");
                            if let Some(ListenerResult::KeyToSink(key, _)) = input_listener.result {
                                println!("found change {:?}", key);
                                self.key = Some(key);
                                input_listener.consume_result();
                                input_listener.stop_listening();
                                changed = true;
                            }
                        } else {
                            if ui.button("rebind").clicked() {
                                input_listener.listen_for_controller_key(sink_id);
                            }
                        }
                    }
                    false
                },
            );
        });

        changed
    }

    fn setup(app: &mut App) {}
}

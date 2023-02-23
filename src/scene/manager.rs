use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use event_bus::{dispatch_event, Event};
use crate::scene::scene::Scene;

pub struct SceneManager {
    scene_map: Arc<Mutex<HashMap<String, Scene>>>
}

impl SceneManager {

    fn new() -> Self {
        Self {
            scene_map: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    fn add_scene(&mut self, name: String, scene: Scene) -> Option<Scene> {

        let mut scene_map = match self.scene_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        scene_map.insert(name, scene)

    }

    fn render_scene(&self, name: String) -> std::io::Result<()> {
        let scene_map = match self.scene_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        let scene: Option<&Scene> = scene_map.get(name.as_str());

        if scene.is_none() {
            panic!("Scene instance does not exist")
        }

        let mut event = ChangeSceneEvent {
            scene: scene.unwrap(),
            cancelled: false,
            reason: None
        };

        dispatch_event!("engine", &mut event);

        Ok(())
    }

}

pub struct ChangeSceneEvent {
    scene: *const Scene,
    cancelled: bool,
    reason: Option<String>
}

impl Event for ChangeSceneEvent {
    fn cancellable(&self) -> bool {
        true
    }

    fn cancelled(&self) -> bool {
        self.cancelled
    }

    fn get_cancelled_reason(&self) -> Option<String> {
        self.reason.clone()
    }

    fn set_cancelled(&mut self, _cancel: bool, reason: Option<String>) {
        self.cancelled = _cancel;
        self.reason = reason
    }

}


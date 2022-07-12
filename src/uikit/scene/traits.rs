use crate::uikit::scene::{Scene, SceneSession, SceneConnectionOptions};

pub trait WindowSceneDelegate {
    fn will_connect(
        &self,
        scene: Scene,
        session: SceneSession,
        options: SceneConnectionOptions
    );
}

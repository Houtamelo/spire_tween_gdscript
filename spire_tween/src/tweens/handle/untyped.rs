use super::*;

#[derive(Debug, Clone)]
pub struct UntypedHandle {
    pub(crate) id: TweenId,
}

impl Default for UntypedHandle {
    fn default() -> Self { Self { id: TweenId(0) } }
}

impl UntypedHandle {
    pub fn new(id: TweenId) -> Self { Self { id } }

    pub fn map_untyped<TMap>(&self, f: impl FnOnce(&AnyTween) -> TMap) -> Option<TMap> {
        TWEENS.inspect(self.id, f)
    }

    pub fn map_mut_untyped<TMap>(&self, f: impl FnOnce(&mut AnyTween) -> TMap) -> Option<TMap> {
        TWEENS.edit(self.id, f)
    }

    pub fn is_valid(&self) -> bool { TWEENS.is_valid(self.id) }

    pub fn kill(self) {
        if let Some(mut tween) = TWEENS.remove(self.id) {
            tween.stop()
        }
    }

    pub fn complete(self) {
        if let Some(mut tween) = TWEENS.remove(self.id) {
            tween.complete();
        }
    }
}

impl UntypedHandle {
    pub fn state(&self) -> Option<TweenState> { self.map_untyped(|tween| tween.get_state()) }

    pub fn set_state(&self, state: TweenState) -> bool {
        self.map_mut_untyped(|tween| tween.set_state(state))
            .is_some()
    }

    pub fn is_playing(&self) -> Option<bool> { self.map_untyped(|tween| tween.is_playing()) }

    pub fn play(&self) -> bool {
        self.map_mut_untyped(|tween| {
            tween.play();
        })
        .is_some()
    }

    pub fn is_paused(&self) -> Option<bool> { self.map_untyped(|tween| tween.is_paused()) }

    pub fn pause(&self) -> bool {
        self.map_mut_untyped(|tween| {
            tween.pause();
        })
        .is_some()
    }

    pub fn is_stopped(&self) -> Option<bool> { self.map_untyped(|tween| tween.is_stopped()) }

    pub fn stop(&self) -> bool {
        self.map_mut_untyped(|tween| {
            tween.stop();
        })
        .is_some()
    }
}

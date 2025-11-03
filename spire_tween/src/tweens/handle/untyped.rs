use super::*;

/*
#[derive(Clone)]
pub struct UntypedHandle(PtrTween);

impl UntypedHandle {
    pub fn new(tween: PtrTween) -> Self { Self(tween) }

    pub fn map_untyped<TMap>(&self, f: impl FnOnce(&AnyTween) -> TMap) -> Option<TMap> {
        TM.inspect(self.id, f)
    }

    pub fn map_mut_untyped<TMap>(&self, f: impl FnOnce(&mut AnyTween) -> TMap) -> Option<TMap> {
        TM.edit(self.id, f)
    }

    pub fn is_valid(&self) -> bool { TM.is_valid(self.id) }

    pub fn kill(self) {
        if let Some(mut tween) = TM.tween_unregister(self.id) {
            tween.stop()
        }
    }

    pub fn force_complete(self) {
        tween_any_mut(self.id, |tween| tween.force_complete()).log_if_err();
    }
}

impl UntypedHandle {
    pub fn state(&self) -> Option<State> { self.map_untyped(|tween| tween.get_state()) }

    pub fn set_state(&self, state: State) -> bool {
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
*/

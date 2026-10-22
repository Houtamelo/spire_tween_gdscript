use super::*;
pub trait SpireDoAudioServer<Marker = ()> {}
impl<Class: Inherits<AudioServer> + Inherits<Object>> SpireDoAudioServer<()>
for Gd<Class> {}
impl<
    T: WithBaseField + Inherits<AudioServer> + Inherits<Object>,
> SpireDoAudioServer<BaseMarker> for T {}

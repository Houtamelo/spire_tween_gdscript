use super::*;
pub trait SpireDoShaderMaterial<Marker = ()> {}
impl<Class: Inherits<ShaderMaterial> + Inherits<Object>> SpireDoShaderMaterial<()>
for Gd<Class> {}
impl<
    T: WithBaseField + Inherits<ShaderMaterial> + Inherits<Object>,
> SpireDoShaderMaterial<BaseMarker> for T {}

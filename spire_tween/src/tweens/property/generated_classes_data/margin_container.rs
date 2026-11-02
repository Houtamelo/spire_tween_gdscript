use super::*;
pub trait SpireDoMarginContainer<Marker = ()> {}
impl<Class: Inherits<MarginContainer> + Inherits<Object>> SpireDoMarginContainer<()>
for Gd<Class> {}
impl<
    T: WithBaseField + Inherits<MarginContainer> + Inherits<Object>,
> SpireDoMarginContainer<BaseMarker> for T {}

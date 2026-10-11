use godot::prelude::*;
use spire_tween::prelude::*;
#[derive(GodotClass)]
#[class(init, base=Node)]
struct ShakeProbe { base: Base<Node> }
#[godot_api]
impl INode for ShakeProbe {
 fn ready(&mut self) {
  for (name,last_step) in [("exact",0.5),("near",0.499999999)] {
   let mut node=Node2D::new_alloc();
   self.base_mut().add_child(&node);
   let mut tween=node.do_shake(5.0,25.0,0.5,30.0,1.0);
   tween.process(0.5,false);
   tween.process(last_step,false);
   godot_print!("SHAKE {} before_final={} state={:?}",name,node.get_position(),tween.get_state());
   if !tween.is_stopped() {tween.process(0.01,false);}
   godot_print!("SHAKE {} after_final={} state={:?}",name,node.get_position(),tween.get_state());
  }
  self.base().get_tree().quit();
 }
}
struct ProbeLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for ProbeLibrary {}

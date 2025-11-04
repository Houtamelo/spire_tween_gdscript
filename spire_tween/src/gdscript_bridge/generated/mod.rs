use super::*;
mod accept_dialog;
mod animated_sprite_2_d;
mod animated_sprite_3_d;
mod animation_player;
mod area_2_d;
mod area_3_d;
mod aspect_ratio_container;
mod audio_stream_player;
mod audio_stream_player_2_d;
mod audio_stream_player_3_d;
mod button;
mod camera_2_d;
mod camera_3_d;
mod canvas_item;
mod canvas_layer;
mod canvas_modulate;
mod character_body_2_d;
mod character_body_3_d;
mod color_rect;
mod confirmation_dialog;
mod control;
mod decal;
mod fog_volume;
mod geometry_instance_3_d;
mod gpu_particles_attractor_3_d;
mod gpu_particles_attractor_box_3_d;
mod gpu_particles_attractor_sphere_3_d;
mod gpu_particles_attractor_vector_field_3_d;
mod label;
mod label_3_d;
mod light_3_d;
mod line_edit;
mod link_button;
mod node_2_d;
mod node_3_d;
mod parallax_background;
mod path_follow_2_d;
mod path_follow_3_d;
mod physical_bone_3_d;
mod range;
mod reflection_probe;
mod rich_text_label;
mod rigid_body_2_d;
mod rigid_body_3_d;
mod skeleton_3_d;
mod sprite_base_3_d;
mod static_body_2_d;
mod static_body_3_d;
mod status_indicator;
mod sub_viewport;
mod text_edit;
mod texture_progress_bar;
mod video_stream_player;
mod viewport;
mod window;
#[allow(unused_imports)]
pub use self::{
    accept_dialog::*, animated_sprite_2_d::*, animated_sprite_3_d::*,
    animation_player::*, area_2_d::*, area_3_d::*, aspect_ratio_container::*,
    audio_stream_player::*, audio_stream_player_2_d::*, audio_stream_player_3_d::*,
    button::*, camera_2_d::*, camera_3_d::*, canvas_item::*, canvas_layer::*,
    canvas_modulate::*, character_body_2_d::*, character_body_3_d::*, color_rect::*,
    confirmation_dialog::*, control::*, decal::*, fog_volume::*,
    geometry_instance_3_d::*, gpu_particles_attractor_3_d::*,
    gpu_particles_attractor_box_3_d::*, gpu_particles_attractor_sphere_3_d::*,
    gpu_particles_attractor_vector_field_3_d::*, label::*, label_3_d::*, light_3_d::*,
    line_edit::*, link_button::*, node_2_d::*, node_3_d::*, parallax_background::*,
    path_follow_2_d::*, path_follow_3_d::*, physical_bone_3_d::*, range::*,
    reflection_probe::*, rich_text_label::*, rigid_body_2_d::*, rigid_body_3_d::*,
    skeleton_3_d::*, sprite_base_3_d::*, static_body_2_d::*, static_body_3_d::*,
    status_indicator::*, sub_viewport::*, text_edit::*, texture_progress_bar::*,
    video_stream_player::*, viewport::*, window::*,
};

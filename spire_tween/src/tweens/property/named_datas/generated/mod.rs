
use super::*;

pub use accept_dialog::*;
#[allow(unused_imports)]
mod accept_dialog;
pub use animated_sprite_2_d::*;
#[allow(unused_imports)]
mod animated_sprite_2_d;
pub use animation_player::*;
#[allow(unused_imports)]
mod animation_player;
pub use area_2_d::*;
#[allow(unused_imports)]
mod area_2_d;
pub use aspect_ratio_container::*;
#[allow(unused_imports)]
mod aspect_ratio_container;
pub use audio_stream_player::*;
#[allow(unused_imports)]
mod audio_stream_player;
pub use audio_stream_player_2_d::*;
#[allow(unused_imports)]
mod audio_stream_player_2_d;
pub use audio_stream_player_3_d::*;
#[allow(unused_imports)]
mod audio_stream_player_3_d;
pub use button::*;
#[allow(unused_imports)]
mod button;
pub use camera_2_d::*;
#[allow(unused_imports)]
mod camera_2_d;
pub use canvas_item::*;
#[allow(unused_imports)]
mod canvas_item;
pub use canvas_layer::*;
#[allow(unused_imports)]
mod canvas_layer;
pub use canvas_modulate::*;
#[allow(unused_imports)]
mod canvas_modulate;
pub use character_body_2_d::*;
#[allow(unused_imports)]
mod character_body_2_d;
pub use color_rect::*;
#[allow(unused_imports)]
mod color_rect;
pub use confirmation_dialog::*;
#[allow(unused_imports)]
mod confirmation_dialog;
pub use control::*;
#[allow(unused_imports)]
mod control;
pub use label::*;
#[allow(unused_imports)]
mod label;
pub use line_edit::*;
#[allow(unused_imports)]
mod line_edit;
pub use link_button::*;
#[allow(unused_imports)]
mod link_button;
pub use node_2_d::*;
#[allow(unused_imports)]
mod node_2_d;
pub use path_follow_2_d::*;
#[allow(unused_imports)]
mod path_follow_2_d;
pub use range::*;
#[allow(unused_imports)]
mod range;
pub use rich_text_label::*;
#[allow(unused_imports)]
mod rich_text_label;
pub use rigid_body_2_d::*;
#[allow(unused_imports)]
mod rigid_body_2_d;
pub use static_body_2_d::*;
#[allow(unused_imports)]
mod static_body_2_d;
pub use status_indicator::*;
#[allow(unused_imports)]
mod status_indicator;
pub use text_edit::*;
#[allow(unused_imports)]
mod text_edit;
pub use texture_progress_bar::*;
#[allow(unused_imports)]
mod texture_progress_bar;
pub use video_stream_player::*;
#[allow(unused_imports)]
mod video_stream_player;
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyF32Data {
    AnimatedSprite2D(AnimatedSprite2DF32Data),
    AnimationPlayer(AnimationPlayerF32Data),
    Area2D(Area2DF32Data),
    AspectRatioContainer(AspectRatioContainerF32Data),
    AudioStreamPlayer(AudioStreamPlayerF32Data),
    AudioStreamPlayer2D(AudioStreamPlayer2DF32Data),
    AudioStreamPlayer3D(AudioStreamPlayer3DF32Data),
    Camera2D(Camera2DF32Data),
    CanvasItem(CanvasItemF32Data),
    CanvasLayer(CanvasLayerF32Data),
    CanvasModulate(CanvasModulateF32Data),
    CharacterBody2D(CharacterBody2DF32Data),
    ColorRect(ColorRectF32Data),
    Control(ControlF32Data),
    Label(LabelF32Data),
    Node2D(Node2DF32Data),
    PathFollow2D(PathFollow2DF32Data),
    RichTextLabel(RichTextLabelF32Data),
    RigidBody2D(RigidBody2DF32Data),
    StaticBody2D(StaticBody2DF32Data),
    TextureProgressBar(TextureProgressBarF32Data),
    VideoStreamPlayer(VideoStreamPlayerF32Data),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyF32Data {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, AnimatedSprite2DF32Data, AnimationPlayerF32Data,
            Area2DF32Data, AspectRatioContainerF32Data, AudioStreamPlayerF32Data,
            AudioStreamPlayer2DF32Data, AudioStreamPlayer3DF32Data, Camera2DF32Data,
            CanvasItemF32Data, CanvasLayerF32Data, CanvasModulateF32Data,
            CharacterBody2DF32Data, ColorRectF32Data, ControlF32Data, LabelF32Data,
            Node2DF32Data, PathFollow2DF32Data, RichTextLabelF32Data, RigidBody2DF32Data,
            StaticBody2DF32Data, TextureProgressBarF32Data, VideoStreamPlayerF32Data
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}


#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyF64Data {
    Range(RangeF64Data),
    TextEdit(TextEditF64Data),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyF64Data {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, RangeF64Data, TextEditF64Data
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}


#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyI32Data {
    AnimatedSprite2D(AnimatedSprite2DI32Data),
    CanvasItem(CanvasItemI32Data),
    Label(LabelI32Data),
    LineEdit(LineEditI32Data),
    RichTextLabel(RichTextLabelI32Data),
    TextEdit(TextEditI32Data),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyI32Data {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, AnimatedSprite2DI32Data, CanvasItemI32Data, LabelI32Data,
            LineEditI32Data, RichTextLabelI32Data, TextEditI32Data
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}


#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyI64Data {
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyI64Data {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner,
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}


#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyVector2Data {
    Area2D(Area2DVector2Data),
    Camera2D(Camera2DVector2Data),
    CanvasLayer(CanvasLayerVector2Data),
    CharacterBody2D(CharacterBody2DVector2Data),
    Control(ControlVector2Data),
    Node2D(Node2DVector2Data),
    RigidBody2D(RigidBody2DVector2Data),
    StaticBody2D(StaticBody2DVector2Data),
    TextureProgressBar(TextureProgressBarVector2Data),
    Follow2D(PropertyVector2Node2DFollowData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyVector2Data {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, Area2DVector2Data, Camera2DVector2Data,
            CanvasLayerVector2Data, CharacterBody2DVector2Data, ControlVector2Data,
            Node2DVector2Data, RigidBody2DVector2Data, StaticBody2DVector2Data,
            TextureProgressBarVector2Data, PropertyVector2Node2DFollowData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}


#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyVector2IData {
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyVector2IData {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner,
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}


#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyVector3Data {
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyVector3Data {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner,
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}


#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyVector3IData {
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyVector3IData {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner,
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}


#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyColorData {
    CanvasItem(CanvasItemColorData),
    CanvasModulate(CanvasModulateColorData),
    ColorRect(ColorRectColorData),
    TextureProgressBar(TextureProgressBarColorData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyColorData {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, CanvasItemColorData, CanvasModulateColorData,
            ColorRectColorData, TextureProgressBarColorData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}


#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyGStringData {
    AcceptDialog(AcceptDialogGStringData),
    Button(ButtonGStringData),
    ConfirmationDialog(ConfirmationDialogGStringData),
    Label(LabelGStringData),
    LineEdit(LineEditGStringData),
    LinkButton(LinkButtonGStringData),
    RichTextLabel(RichTextLabelGStringData),
    StatusIndicator(StatusIndicatorGStringData),
    TextEdit(TextEditGStringData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyGStringData {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, AcceptDialogGStringData, ButtonGStringData,
            ConfirmationDialogGStringData, LabelGStringData, LineEditGStringData,
            LinkButtonGStringData, RichTextLabelGStringData, StatusIndicatorGStringData,
            TextEditGStringData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}



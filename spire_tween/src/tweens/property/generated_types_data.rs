use super::*;
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyDataInt {
    AnimatedSprite2D(AnimatedSprite2DIntData),
    CanvasItem(CanvasItemIntData),
    Label(LabelIntData),
    LineEdit(LineEditIntData),
    RichTextLabel(RichTextLabelIntData),
    TextEdit(TextEditIntData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataInt {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, AnimatedSprite2DIntData, CanvasItemIntData, LabelIntData,
            LineEditIntData, RichTextLabelIntData, TextEditIntData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyDataFloat {
    AnimatedSprite2D(AnimatedSprite2DFloatData),
    AnimationPlayer(AnimationPlayerFloatData),
    Area2D(Area2DFloatData),
    AspectRatioContainer(AspectRatioContainerFloatData),
    AudioStreamPlayer(AudioStreamPlayerFloatData),
    AudioStreamPlayer2D(AudioStreamPlayer2DFloatData),
    AudioStreamPlayer3D(AudioStreamPlayer3DFloatData),
    Camera2D(Camera2DFloatData),
    CanvasItem(CanvasItemFloatData),
    CanvasLayer(CanvasLayerFloatData),
    CanvasModulate(CanvasModulateFloatData),
    CharacterBody2D(CharacterBody2DFloatData),
    ColorRect(ColorRectFloatData),
    Control(ControlFloatData),
    Label(LabelFloatData),
    Node2D(Node2DFloatData),
    PathFollow2D(PathFollow2DFloatData),
    Range(RangeFloatData),
    RichTextLabel(RichTextLabelFloatData),
    RigidBody2D(RigidBody2DFloatData),
    StaticBody2D(StaticBody2DFloatData),
    TextEdit(TextEditFloatData),
    TextureProgressBar(TextureProgressBarFloatData),
    VideoStreamPlayer(VideoStreamPlayerFloatData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataFloat {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, AnimatedSprite2DFloatData, AnimationPlayerFloatData,
            Area2DFloatData, AspectRatioContainerFloatData, AudioStreamPlayerFloatData,
            AudioStreamPlayer2DFloatData, AudioStreamPlayer3DFloatData,
            Camera2DFloatData, CanvasItemFloatData, CanvasLayerFloatData,
            CanvasModulateFloatData, CharacterBody2DFloatData, ColorRectFloatData,
            ControlFloatData, LabelFloatData, Node2DFloatData, PathFollow2DFloatData,
            RangeFloatData, RichTextLabelFloatData, RigidBody2DFloatData,
            StaticBody2DFloatData, TextEditFloatData, TextureProgressBarFloatData,
            VideoStreamPlayerFloatData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyDataVector2 {
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
impl IGeneralPropertyData for PropertyDataVector2 {
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
pub enum PropertyDataVector2i {
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataVector2i {
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
pub enum PropertyDataVector3 {
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataVector3 {
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
pub enum PropertyDataVector3i {
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataVector3i {
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
pub enum PropertyDataColor {
    CanvasItem(CanvasItemColorData),
    CanvasModulate(CanvasModulateColorData),
    ColorRect(ColorRectColorData),
    TextureProgressBar(TextureProgressBarColorData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataColor {
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
pub enum PropertyDataString {
    AcceptDialog(AcceptDialogStringData),
    Button(ButtonStringData),
    ConfirmationDialog(ConfirmationDialogStringData),
    Label(LabelStringData),
    LineEdit(LineEditStringData),
    LinkButton(LinkButtonStringData),
    RichTextLabel(RichTextLabelStringData),
    StatusIndicator(StatusIndicatorStringData),
    TextEdit(TextEditStringData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataString {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, AcceptDialogStringData, ButtonStringData,
            ConfirmationDialogStringData, LabelStringData, LineEditStringData,
            LinkButtonStringData, RichTextLabelStringData, StatusIndicatorStringData,
            TextEditStringData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}

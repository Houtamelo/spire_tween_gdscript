use super::*;
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyDataInt {
    AnimatedSprite2D(AnimatedSprite2DIntData),
    AnimatedSprite3D(AnimatedSprite3DIntData),
    CanvasItem(CanvasItemIntData),
    Label(LabelIntData),
    Label3D(Label3DIntData),
    LineEdit(LineEditIntData),
    RichTextLabel(RichTextLabelIntData),
    SubViewport(SubViewportIntData),
    TextEdit(TextEditIntData),
    Window(WindowIntData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataInt {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, AnimatedSprite2DIntData, AnimatedSprite3DIntData,
            CanvasItemIntData, LabelIntData, Label3DIntData, LineEditIntData,
            RichTextLabelIntData, SubViewportIntData, TextEditIntData, WindowIntData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyDataFloat {
    AnimatedSprite2D(AnimatedSprite2DFloatData),
    AnimatedSprite3D(AnimatedSprite3DFloatData),
    AnimationPlayer(AnimationPlayerFloatData),
    Area2D(Area2DFloatData),
    Area3D(Area3DFloatData),
    AspectRatioContainer(AspectRatioContainerFloatData),
    AudioStreamPlayer(AudioStreamPlayerFloatData),
    AudioStreamPlayer2D(AudioStreamPlayer2DFloatData),
    AudioStreamPlayer3D(AudioStreamPlayer3DFloatData),
    Camera2D(Camera2DFloatData),
    Camera3D(Camera3DFloatData),
    CanvasItem(CanvasItemFloatData),
    CanvasLayer(CanvasLayerFloatData),
    CanvasModulate(CanvasModulateFloatData),
    CharacterBody2D(CharacterBody2DFloatData),
    CharacterBody3D(CharacterBody3DFloatData),
    ColorRect(ColorRectFloatData),
    Control(ControlFloatData),
    Decal(DecalFloatData),
    FogVolume(FogVolumeFloatData),
    GeometryInstance3D(GeometryInstance3DFloatData),
    GpuParticlesAttractor3D(GpuParticlesAttractor3DFloatData),
    GpuParticlesAttractorBox3D(GpuParticlesAttractorBox3DFloatData),
    GpuParticlesAttractorSphere3D(GpuParticlesAttractorSphere3DFloatData),
    GpuParticlesAttractorVectorField3D(GpuParticlesAttractorVectorField3DFloatData),
    Label(LabelFloatData),
    Label3D(Label3DFloatData),
    Light3D(Light3DFloatData),
    Node2D(Node2DFloatData),
    Node3D(Node3DFloatData),
    ParallaxBackground(ParallaxBackgroundFloatData),
    PathFollow2D(PathFollow2DFloatData),
    PathFollow3D(PathFollow3DFloatData),
    PhysicalBone3D(PhysicalBone3DFloatData),
    Range(RangeFloatData),
    ReflectionProbe(ReflectionProbeFloatData),
    RichTextLabel(RichTextLabelFloatData),
    RigidBody2D(RigidBody2DFloatData),
    RigidBody3D(RigidBody3DFloatData),
    Skeleton3D(Skeleton3DFloatData),
    SpriteBase3D(SpriteBase3DFloatData),
    StaticBody2D(StaticBody2DFloatData),
    StaticBody3D(StaticBody3DFloatData),
    TextEdit(TextEditFloatData),
    TextureProgressBar(TextureProgressBarFloatData),
    VideoStreamPlayer(VideoStreamPlayerFloatData),
    Viewport(ViewportFloatData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataFloat {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, AnimatedSprite2DFloatData, AnimatedSprite3DFloatData,
            AnimationPlayerFloatData, Area2DFloatData, Area3DFloatData,
            AspectRatioContainerFloatData, AudioStreamPlayerFloatData,
            AudioStreamPlayer2DFloatData, AudioStreamPlayer3DFloatData,
            Camera2DFloatData, Camera3DFloatData, CanvasItemFloatData,
            CanvasLayerFloatData, CanvasModulateFloatData, CharacterBody2DFloatData,
            CharacterBody3DFloatData, ColorRectFloatData, ControlFloatData,
            DecalFloatData, FogVolumeFloatData, GeometryInstance3DFloatData,
            GpuParticlesAttractor3DFloatData, GpuParticlesAttractorBox3DFloatData,
            GpuParticlesAttractorSphere3DFloatData,
            GpuParticlesAttractorVectorField3DFloatData, LabelFloatData,
            Label3DFloatData, Light3DFloatData, Node2DFloatData, Node3DFloatData,
            ParallaxBackgroundFloatData, PathFollow2DFloatData, PathFollow3DFloatData,
            PhysicalBone3DFloatData, RangeFloatData, ReflectionProbeFloatData,
            RichTextLabelFloatData, RigidBody2DFloatData, RigidBody3DFloatData,
            Skeleton3DFloatData, SpriteBase3DFloatData, StaticBody2DFloatData,
            StaticBody3DFloatData, TextEditFloatData, TextureProgressBarFloatData,
            VideoStreamPlayerFloatData, ViewportFloatData
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
    Camera3D(Camera3DVector2Data),
    CanvasLayer(CanvasLayerVector2Data),
    CharacterBody2D(CharacterBody2DVector2Data),
    Control(ControlVector2Data),
    Label3D(Label3DVector2Data),
    Node2D(Node2DVector2Data),
    ParallaxBackground(ParallaxBackgroundVector2Data),
    RigidBody2D(RigidBody2DVector2Data),
    SpriteBase3D(SpriteBase3DVector2Data),
    StaticBody2D(StaticBody2DVector2Data),
    TextureProgressBar(TextureProgressBarVector2Data),
    Follow2D(PropertyVector2Node2DFollowData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataVector2 {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, Area2DVector2Data, Camera2DVector2Data,
            Camera3DVector2Data, CanvasLayerVector2Data, CharacterBody2DVector2Data,
            ControlVector2Data, Label3DVector2Data, Node2DVector2Data,
            ParallaxBackgroundVector2Data, RigidBody2DVector2Data,
            SpriteBase3DVector2Data, StaticBody2DVector2Data,
            TextureProgressBarVector2Data, PropertyVector2Node2DFollowData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyDataVector2i {
    SubViewport(SubViewportVector2iData),
    Window(WindowVector2iData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataVector2i {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, SubViewportVector2iData, WindowVector2iData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyDataVector3 {
    Area3D(Area3DVector3Data),
    CharacterBody3D(CharacterBody3DVector3Data),
    Decal(DecalVector3Data),
    FogVolume(FogVolumeVector3Data),
    GpuParticlesAttractorBox3D(GpuParticlesAttractorBox3DVector3Data),
    GpuParticlesAttractorVectorField3D(GpuParticlesAttractorVectorField3DVector3Data),
    Node3D(Node3DVector3Data),
    PhysicalBone3D(PhysicalBone3DVector3Data),
    ReflectionProbe(ReflectionProbeVector3Data),
    RigidBody3D(RigidBody3DVector3Data),
    StaticBody3D(StaticBody3DVector3Data),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataVector3 {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, Area3DVector3Data, CharacterBody3DVector3Data,
            DecalVector3Data, FogVolumeVector3Data,
            GpuParticlesAttractorBox3DVector3Data,
            GpuParticlesAttractorVectorField3DVector3Data, Node3DVector3Data,
            PhysicalBone3DVector3Data, ReflectionProbeVector3Data,
            RigidBody3DVector3Data, StaticBody3DVector3Data
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
    Decal(DecalColorData),
    Label3D(Label3DColorData),
    Light3D(Light3DColorData),
    ReflectionProbe(ReflectionProbeColorData),
    SpriteBase3D(SpriteBase3DColorData),
    TextureProgressBar(TextureProgressBarColorData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataColor {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, CanvasItemColorData, CanvasModulateColorData,
            ColorRectColorData, DecalColorData, Label3DColorData, Light3DColorData,
            ReflectionProbeColorData, SpriteBase3DColorData, TextureProgressBarColorData
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
    Label3D(Label3DStringData),
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
            ConfirmationDialogStringData, LabelStringData, Label3DStringData,
            LineEditStringData, LinkButtonStringData, RichTextLabelStringData,
            StatusIndicatorStringData, TextEditStringData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}

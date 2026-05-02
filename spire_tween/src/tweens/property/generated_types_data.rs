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
    Sprite2D(Sprite2DIntData),
    SubViewport(SubViewportIntData),
    TextEdit(TextEditIntData),
    Window(WindowIntData),
    ViaCallable(PropertyDataViaCallable<i64>),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataInt {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, AnimatedSprite2DIntData, AnimatedSprite3DIntData,
            CanvasItemIntData, LabelIntData, Label3DIntData, LineEditIntData,
            RichTextLabelIntData, Sprite2DIntData, SubViewportIntData, TextEditIntData,
            WindowIntData, PropertyDataViaCallable < i64 >
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
    ViaCallable(PropertyDataViaCallable<f64>),
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
            VideoStreamPlayerFloatData, ViewportFloatData, PropertyDataViaCallable < f64
            >
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyDataVec2 {
    Area2D(Area2DVec2Data),
    Camera2D(Camera2DVec2Data),
    Camera3D(Camera3DVec2Data),
    CanvasLayer(CanvasLayerVec2Data),
    CharacterBody2D(CharacterBody2DVec2Data),
    Control(ControlVec2Data),
    Label3D(Label3DVec2Data),
    Node2D(Node2DVec2Data),
    ParallaxBackground(ParallaxBackgroundVec2Data),
    RigidBody2D(RigidBody2DVec2Data),
    SpriteBase3D(SpriteBase3DVec2Data),
    StaticBody2D(StaticBody2DVec2Data),
    TextureProgressBar(TextureProgressBarVec2Data),
    ViaCallable(PropertyDataViaCallable<Vector2>),
    Follow2D(PropertyVec2Node2DFollowData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataVec2 {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, Area2DVec2Data, Camera2DVec2Data, Camera3DVec2Data,
            CanvasLayerVec2Data, CharacterBody2DVec2Data, ControlVec2Data,
            Label3DVec2Data, Node2DVec2Data, ParallaxBackgroundVec2Data,
            RigidBody2DVec2Data, SpriteBase3DVec2Data, StaticBody2DVec2Data,
            TextureProgressBarVec2Data, PropertyDataViaCallable < Vector2 >,
            PropertyVec2Node2DFollowData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyDataVec2i {
    SubViewport(SubViewportVec2iData),
    Window(WindowVec2iData),
    ViaCallable(PropertyDataViaCallable<Vector2i>),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataVec2i {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, SubViewportVec2iData, WindowVec2iData,
            PropertyDataViaCallable < Vector2i >
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyDataVec3 {
    Area3D(Area3DVec3Data),
    CharacterBody3D(CharacterBody3DVec3Data),
    Decal(DecalVec3Data),
    FogVolume(FogVolumeVec3Data),
    GpuParticlesAttractorBox3D(GpuParticlesAttractorBox3DVec3Data),
    GpuParticlesAttractorVectorField3D(GpuParticlesAttractorVectorField3DVec3Data),
    Node3D(Node3DVec3Data),
    PhysicalBone3D(PhysicalBone3DVec3Data),
    ReflectionProbe(ReflectionProbeVec3Data),
    RigidBody3D(RigidBody3DVec3Data),
    StaticBody3D(StaticBody3DVec3Data),
    ViaCallable(PropertyDataViaCallable<Vector3>),
    Follow3D(PropertyVec3Node3DFollowData),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataVec3 {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, Area3DVec3Data, CharacterBody3DVec3Data, DecalVec3Data,
            FogVolumeVec3Data, GpuParticlesAttractorBox3DVec3Data,
            GpuParticlesAttractorVectorField3DVec3Data, Node3DVec3Data,
            PhysicalBone3DVec3Data, ReflectionProbeVec3Data, RigidBody3DVec3Data,
            StaticBody3DVec3Data, PropertyDataViaCallable < Vector3 >,
            PropertyVec3Node3DFollowData
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}
#[derive(Debug, Clone)]
#[allow(unused)]
#[delegated_enum(impl_conversions)]
pub enum PropertyDataVec3i {
    ViaCallable(PropertyDataViaCallable<Vector3i>),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataVec3i {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, PropertyDataViaCallable < Vector3i >
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
    ViaCallable(PropertyDataViaCallable<Color>),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataColor {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, CanvasItemColorData, CanvasModulateColorData,
            ColorRectColorData, DecalColorData, Label3DColorData, Light3DColorData,
            ReflectionProbeColorData, SpriteBase3DColorData, TextureProgressBarColorData,
            PropertyDataViaCallable < Color >
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
    ViaCallable(PropertyDataViaCallable<GString>),
    Custom(PropertyDataCustom),
}
impl IGeneralPropertyData for PropertyDataString {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        nested_try_from_path_and_object! {
            _path_str, owner, AcceptDialogStringData, ButtonStringData,
            ConfirmationDialogStringData, LabelStringData, Label3DStringData,
            LineEditStringData, LinkButtonStringData, RichTextLabelStringData,
            StatusIndicatorStringData, TextEditStringData, PropertyDataViaCallable <
            GString >
        }
        Self::Custom(PropertyDataCustom::from_path_and_owner(_path_str, path, owner))
    }
}

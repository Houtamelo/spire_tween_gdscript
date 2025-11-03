use super::*;

pub trait ILerpable: Sized {
    type BasicLerper: BasicLerp<Self>;
    type Lerper: SpireLerp<Self>;
}

/*
impl ILerpable for i32 {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for f32 {
    type BasicLerper = ();
    type Lerper = ();
}
*/

impl ILerpable for i64 {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for f64 {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for GString {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Color {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Vector2 {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Vector2i {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Vector3 {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Vector3i {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Variant {
    type BasicLerper = CustomBasicLerper;
    type Lerper = CustomLerper;
}

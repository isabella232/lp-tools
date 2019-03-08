use libc::{c_char, c_void, size_t, ssize_t};

// MagickCore/method-attribute.h
#[allow(non_upper_case_globals)]
pub const MagickPathExtent: size_t = 4096;

// MagickCore/geometry.h
#[repr(C)]
pub enum GravityType {
    UndefinedGravity,
    // ForgetGravity = 0,
    NorthWestGravity = 1,
    NorthGravity = 2,
    NorthEastGravity = 3,
    WestGravity = 4,
    CenterGravity = 5,
    EastGravity = 6,
    SouthWestGravity = 7,
    SouthGravity = 8,
    SouthEastGravity = 9,
}

// MagickCore/magick-type.h
#[repr(C)]
pub enum MagickBooleanType {
    MagickFalse = 0,
    MagickTrue = 1,
}

// MagickCore/resample.h
#[repr(C)]
pub enum FilterType {
    UndefinedFilter,
    PointFilter,
    BoxFilter,
    TriangleFilter,
    HermiteFilter,
    HannFilter,
    HammingFilter,
    BlackmanFilter,
    GaussianFilter,
    QuadraticFilter,
    CubicFilter,
    CatromFilter,
    MitchellFilter,
    JincFilter,
    SincFilter,
    SincFastFilter,
    KaiserFilter,
    WelchFilter,
    ParzenFilter,
    BohmanFilter,
    BartlettFilter,
    LagrangeFilter,
    LanczosFilter,
    LanczosSharpFilter,
    Lanczos2Filter,
    Lanczos2SharpFilter,
    RobidouxFilter,
    RobidouxSharpFilter,
    CosineFilter,
    SplineFilter,
    LanczosRadiusFilter,
    SentinelFilter,
}

// MagickWand/MagickWand.h
#[repr(C)]
pub struct MagickWand {
    id: size_t,
    name: [c_char; MagickPathExtent],
    images: *mut c_void,     // Image *
    image_info: *mut c_void, // ImageInfo *
    exception: *mut c_void,  // ExceptionInfo *
    insert_before: MagickBooleanType,
    image_pending: MagickBooleanType,
    debug: MagickBooleanType,
    signature: size_t,
}

#[link(name = "MagickWand-7.Q16HDRI")]
extern "C" {
    // MagickWand/MagickWand.h
    pub fn NewMagickWand() -> *mut MagickWand;
    pub fn MagickWandGenesis();
    pub fn MagickWandTerminus();
    pub fn DestroyMagickWand(wand: *mut MagickWand) -> *mut MagickWand;

    // MagickWand/magick-image.h
    pub fn MagickCropImage(
        wand: *mut MagickWand,
        width: size_t,
        height: size_t,
        x: ssize_t,
        y: ssize_t,
    ) -> MagickBooleanType;
    pub fn MagickReadImage(wand: *mut MagickWand, filename: *const c_char) -> MagickBooleanType;
    pub fn MagickGetImageGravity(wand: *mut MagickWand) -> GravityType;
    pub fn MagickGetImageHeight(wand: *mut MagickWand) -> size_t;
    pub fn MagickGetImageWidth(wand: *mut MagickWand) -> size_t;
    pub fn MagickResizeImage(
        wand: *mut MagickWand,
        columns: size_t,
        rows: size_t,
        filter: FilterType,
    ) -> MagickBooleanType;
    pub fn MagickSetImageExtent(
        wand: *mut MagickWand,
        columns: size_t,
        rows: size_t,
    ) -> MagickBooleanType;
    pub fn MagickSetImageGravity(wand: *mut MagickWand, gravity: GravityType) -> MagickBooleanType;
    pub fn MagickWriteImage(wand: *mut MagickWand, filename: *const c_char) -> MagickBooleanType;
}

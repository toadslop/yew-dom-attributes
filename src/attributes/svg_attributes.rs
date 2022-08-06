use std::fmt::Display;

use strum::Display;

use crate::attribute_holder::Attribute;

use super::html_attributes::AriaRole;

/// This enum models the attributes for SVG elements.
#[derive(Debug, Clone, Display, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum SVGAttributes {
    // Note: class is already handled well by Yew, so we are disabling it here.
    // Class(String),
    Color(String),
    Height(NumberOrString),
    Id(String),
    Lang(String),
    Max(NumberOrString),
    Media(String),
    Method(String),
    Min(NumberOrString),
    Name(String),
    // Note: implementing css properties is unneccessary since style tags are already handled by Yew and Stylist
    // Style: CSSProperties
    Target(String),
    Type(String),
    Width(NumberOrString),
    // Other HTML properties supported by SVG elements in browsers
    Role(AriaRole),
    TabIndex(i64),
    CrossOrigin(CrossOrigin),
    // SVG Specific attributes
    AccentHeight(NumberOrString),
    Accumulate(Accumulate),
    Additive(Additive),
    AlignmentBaseline(AlignmentBaseline),
    #[strum(serialize = "allowReorder")]
    AllowReorder(AllowReorder),
    Alphabetic(NumberOrString),
    Amplitude(NumberOrString),
    ArabicForm(ArabicForm),
    Ascent(NumberOrString),
    #[strum(serialize = "attributeName")]
    AttributeName(String),
    #[strum(serialize = "attributeType")]
    AttributeType(String),
    AutoReverse(bool),
    Azimuth(NumberOrString),
    #[strum(serialize = "baseFrequency")]
    BaseFrequency(NumberOrString),
    BaselineShift(NumberOrString),
    #[strum(serialize = "baseProfile")]
    BaseProfile(NumberOrString),
    Bbox(NumberOrString),
    Begin(NumberOrString),
    Bias(NumberOrString),
    By(NumberOrString),
    CalcMode(NumberOrString),
    CapHeight(NumberOrString),
    Clip(NumberOrString),
    #[strum(serialize = "clipPath")]
    ClipPath(String),
    #[strum(serialize = "clipPathUnits")]
    ClipPathUnits(ClipPathUnits),
    ClipRule(ClipRule),
    ColorInterpolation(NumberOrString),
    ColorInterpolationFilters(ColorInterpolationFilters),
    ColorProfile(NumberOrString),
    ColorRendering(NumberOrString),
    #[strum(serialize = "contentScriptType")]
    ContentScriptType(NumberOrString),
    #[strum(serialize = "contentStyleType")]
    ContentStyleType(NumberOrString),
    Cursor(NumberOrString),
    Cx(NumberOrString),
    Cy(NumberOrString),
    D(String),
    Decelerate(NumberOrString),
    Descent(NumberOrString),
    #[strum(serialize = "diffuseConstant")]
    DiffuseConstant(NumberOrString),
    Direction(NumberOrString),
    Display(NumberOrString),
    Divisor(NumberOrString),
    DominantBaseline(NumberOrString),
    Dur(NumberOrString),
    Dx(NumberOrString),
    Dy(NumberOrString),
    #[strum(serialize = "edgeMode")]
    EdgeMode(NumberOrString),
    Elevation(NumberOrString),
    EnableBackground(NumberOrString),
    End(NumberOrString),
    Exponent(NumberOrString),
    #[strum(serialize = "externalResourcesRequired")]
    ExternalResourcesRequired(bool),
    Fill(String),
    FillOpacity(NumberOrString),
    FillRule(FillRule),
    Filter(String),
    #[strum(serialize = "filterRes")]
    FilterRes(NumberOrString),
    #[strum(serialize = "filterUnits")]
    FilterUnits(NumberOrString),
    FloodColor(NumberOrString),
    FloodOpacity(NumberOrString),
    Focusable(Focusable),
    FontFamily(String),
    FontSize(NumberOrString),
    FontSizeAdjust(NumberOrString),
    FontStretch(NumberOrString),
    FontStyle(NumberOrString),
    FontVariant(NumberOrString),
    FontWeight(NumberOrString),
    Format(NumberOrString),
    Fr(NumberOrString),
    From(NumberOrString),
    Fx(NumberOrString),
    Fy(NumberOrString),
    G1(NumberOrString),
    G2(NumberOrString),
    GlyphName(NumberOrString),
    GlyphOrientationHorizontal(NumberOrString),
    GlyphOrientationVertical(NumberOrString),
    #[strum(serialize = "glyphRef")]
    GlyphRef(NumberOrString),
    #[strum(serialize = "gradientTransform")]
    GradientTransform(String),
    #[strum(serialize = "gradientUnits")]
    GradientUnits(String),
    Hanging(NumberOrString),
    #[strum(serialize = "horizAdvX")]
    HorizAdvX(NumberOrString),
    HorizOriginX(NumberOrString),
    Href(String),
    Ideographic(NumberOrString),
    ImageRendering(NumberOrString),
    In2(NumberOrString),
    In(String),
    Intercept(NumberOrString),
    K1(NumberOrString),
    K2(NumberOrString),
    K3(NumberOrString),
    K4(NumberOrString),
    K(NumberOrString),
    #[strum(serialize = "kernelMatrix")]
    KernelMatrix(NumberOrString),
    #[strum(serialize = "kernelUnitLength")]
    KernelUnitLength(NumberOrString),
    Kerning(NumberOrString),
    #[strum(serialize = "keyPoints")]
    KeyPoints(NumberOrString),
    #[strum(serialize = "keySplines")]
    KeySplines(NumberOrString),
    #[strum(serialize = "keyTimes")]
    KeyTimes(NumberOrString),
    #[strum(serialize = "lengthAdjust")]
    LengthAdjust(NumberOrString),
    LetterSpacing(NumberOrString),
    LightingColor(NumberOrString),
    #[strum(serialize = "limitingConeAngle")]
    LimitingConeAngle(NumberOrString),
    Local(NumberOrString),
    MarkerEnd(String),
    #[strum(serialize = "markerHeight")]
    MarkerHeight(NumberOrString),
    MarkerMid(String),
    MarkerStart(String),
    #[strum(serialize = "markerUnits")]
    MarkerUnits(NumberOrString),
    #[strum(serialize = "markerWidth")]
    MarkerWidth(NumberOrString),
    Mask(String),
    #[strum(serialize = "maskContentUnits")]
    MaskContentUnits(NumberOrString),
    #[strum(serialize = "maskUnits")]
    MaskUnits(NumberOrString),
    Mathematical(NumberOrString),
    Mode(NumberOrString),
    #[strum(serialize = "numOctaves")]
    NumOctaves(NumberOrString),
    Offset(NumberOrString),
    Opacity(NumberOrString),
    Operator(NumberOrString),
    Order(NumberOrString),
    Orient(NumberOrString),
    Orientation(NumberOrString),
    Origin(NumberOrString),
    Overflow(NumberOrString),
    OverlinePosition(NumberOrString),
    PverlineThickness(NumberOrString),
    PaintOrder(NumberOrString),
    #[strum(serialize = "panose-1")]
    Panose1(NumberOrString),
    Path(String),
    #[strum(serialize = "pathLength")]
    PathLength(NumberOrString),
    #[strum(serialize = "patternContentUnits")]
    PatternContentUnits(String),
    #[strum(serialize = "patternTransform")]
    PatternTransform(NumberOrString),
    #[strum(serialize = "patternUnits")]
    PatternUnits(String),
    PointerEvents(NumberOrString),
    Points(String),
    #[strum(serialize = "pointsAtX")]
    PointsAtX(NumberOrString),
    #[strum(serialize = "pointsAtY")]
    PointsAtY(NumberOrString),
    #[strum(serialize = "pointsAtZ")]
    PointsAtZ(NumberOrString),
    #[strum(serialize = "preserveAlpha")]
    PreserveAlpha(bool),
    #[strum(serialize = "preserveAspectRatio")]
    OreserveAspectRatio(String),
    #[strum(serialize = "primitiveUnits")]
    PrimitiveUnits(NumberOrString),
    R(NumberOrString),
    Radius(NumberOrString),
    #[strum(serialize = "refX")]
    RefX(NumberOrString),
    #[strum(serialize = "refY")]
    RefY(NumberOrString),
    RenderingIntent(NumberOrString),
    #[strum(serialize = "repeatCount")]
    RepeatCount(NumberOrString),
    #[strum(serialize = "repeatDur")]
    RepeatDur(NumberOrString),
    #[strum(serialize = "requiredExtensions")]
    RequiredExtensions(NumberOrString),
    #[strum(serialize = "requiredFeatures")]
    RequiredFeatures(NumberOrString),
    Restart(NumberOrString),
    Result(String),
    Rotate(NumberOrString),
    Rx(NumberOrString),
    Ry(NumberOrString),
    Scale(NumberOrString),
    Seed(NumberOrString),
    ShapeRendering(NumberOrString),
    Slope(NumberOrString),
    Spacing(NumberOrString),
    #[strum(serialize = "requiredFeatures")]
    SpecularConstant(NumberOrString),
    #[strum(serialize = "specularExponent")]
    SpecularExponent(NumberOrString),
    Speed(NumberOrString),
    #[strum(serialize = "spreadMethod")]
    SpreadMethod(String),
    #[strum(serialize = "startOffset")]
    StartOffset(NumberOrString),
    #[strum(serialize = "stdDeviation")]
    StdDeviation(NumberOrString),
    Stemh(NumberOrString),
    Stemv(NumberOrString),
    #[strum(serialize = "stitchTiles")]
    StitchTiles(NumberOrString),
    StopColor(String),
    StopOpacity(NumberOrString),
    StrikethroughPosition(NumberOrString),
    StrikethroughThickness(NumberOrString),
    String(NumberOrString),
    Stroke(String),
    StrokeDasharray(NumberOrString),
    StrokeDashoffset(NumberOrString),
    StrokeLinecap(StrokeLinecap),
    StrokeLinejoin(StrokeLinejoin),
    StrokeMiterlimit(NumberOrString),
    StrokeOpacity(NumberOrString),
    StrokeWidth(NumberOrString),
    #[strum(serialize = "surfaceScale")]
    SurfaceScale(NumberOrString),
    #[strum(serialize = "systemLanguage")]
    SystemLanguage(NumberOrString),
    #[strum(serialize = "tableValues")]
    TableValues(NumberOrString),
    #[strum(serialize = "targetX")]
    TargetX(NumberOrString),
    #[strum(serialize = "targetY")]
    TargetY(NumberOrString),
    TextAnchor(String),
    TextDecoration(NumberOrString),
    #[strum(serialize = "textLength")]
    TextLength(NumberOrString),
    TextRendering(NumberOrString),
    To(NumberOrString),
    Transform(String),
    U1(NumberOrString),
    U2(NumberOrString),
    EnderlinePosition(NumberOrString),
    UnderlineThickness(NumberOrString),
    Unicode(NumberOrString),
    UnicodeBidi(NumberOrString),
    UnicodeRange(NumberOrString),
    UnitsPerEm(NumberOrString),
    VAlphabetic(NumberOrString),
    Values(String),
    VectorEffect(NumberOrString),
    Version(String),
    VertAdvY(NumberOrString),
    VertOriginX(NumberOrString),
    VertOriginY(NumberOrString),
    VHanging(NumberOrString),
    VIdeographic(NumberOrString),
    #[strum(serialize = "viewBox")]
    SiewBox(String),
    #[strum(serialize = "viewTarget")]
    ViewTarget(NumberOrString),
    Visibility(NumberOrString),
    VMathematical(NumberOrString),
    Widths(NumberOrString),
    WordSpacing(NumberOrString),
    WritingMode(NumberOrString),
    #[strum(serialize = "x1")]
    X1(NumberOrString),
    #[strum(serialize = "x2")]
    X2(NumberOrString),
    X(NumberOrString),
    #[strum(serialize = "xChannelSelector")]
    XChannelSelector(String),
    XHeight(NumberOrString),
    #[strum(serialize = "xlink:actuate")]
    XlinkActuate(String),
    #[strum(serialize = "xlink:arcrole")]
    XlinkArcrole(String),
    #[strum(serialize = "xlink:href")]
    XlinkHref(String),
    #[strum(serialize = "xlink:role")]
    XlinkRole(String),
    #[strum(serialize = "xlink:show")]
    XlinkShow(String),
    #[strum(serialize = "xlink:title")]
    XlinkTitle(String),
    #[strum(serialize = "xlink:type")]
    XlinkType(String),
    #[strum(serialize = "xml:base")]
    XmlBase(String),
    #[strum(serialize = "xml:lang")]
    XmlLang(String),
    Xmlns(String),
    #[strum(serialize = "xmlns:xlink")]
    XmlnsXlink(String),
    #[strum(serialize = "xml:space")]
    XmlSpace(String),
    #[strum(serialize = "y1")]
    Y1(NumberOrString),
    #[strum(serialize = "y2")]
    Y2(NumberOrString),
    Y(NumberOrString),
    #[strum(serialize = "yChannelSelector ")]
    YChannelSelector(String),
    Z(NumberOrString),
    #[strum(serialize = "zoomAndPan")]
    ZoomAndPan(String),
}

// Attributes must be equal to each other based on their key rather than their value.
impl PartialEq for SVGAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Attribute for SVGAttributes {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match self {
            SVGAttributes::Color(val) => Some(val.to_string()),
            SVGAttributes::Height(val) => Some(val.to_string()),
            SVGAttributes::Id(val) => Some(val.to_string()),
            SVGAttributes::Lang(val) => Some(val.to_string()),
            SVGAttributes::Max(val) => Some(val.to_string()),
            SVGAttributes::Media(val) => Some(val.to_string()),
            SVGAttributes::Method(val) => Some(val.to_string()),
            SVGAttributes::Min(val) => Some(val.to_string()),
            SVGAttributes::Name(val) => Some(val.to_string()),
            SVGAttributes::Target(val) => Some(val.to_string()),
            SVGAttributes::Type(val) => Some(val.to_string()),
            SVGAttributes::Width(val) => Some(val.to_string()),
            SVGAttributes::Role(val) => Some(val.to_string()),
            SVGAttributes::TabIndex(val) => Some(val.to_string()),
            SVGAttributes::CrossOrigin(val) => Some(val.to_string()),
            SVGAttributes::AccentHeight(val) => Some(val.to_string()),
            SVGAttributes::Accumulate(val) => Some(val.to_string()),
            SVGAttributes::Additive(val) => Some(val.to_string()),
            SVGAttributes::AlignmentBaseline(val) => Some(val.to_string()),
            SVGAttributes::AllowReorder(val) => Some(val.to_string()),
            SVGAttributes::Alphabetic(val) => Some(val.to_string()),
            SVGAttributes::Amplitude(val) => Some(val.to_string()),
            SVGAttributes::ArabicForm(val) => Some(val.to_string()),
            SVGAttributes::Ascent(val) => Some(val.to_string()),
            SVGAttributes::AttributeName(val) => Some(val.to_string()),
            SVGAttributes::AttributeType(val) => Some(val.to_string()),
            SVGAttributes::AutoReverse(val) => Some(val.to_string()),
            SVGAttributes::Azimuth(val) => Some(val.to_string()),
            SVGAttributes::BaseFrequency(val) => Some(val.to_string()),
            SVGAttributes::BaselineShift(val) => Some(val.to_string()),
            SVGAttributes::BaseProfile(val) => Some(val.to_string()),
            SVGAttributes::Bbox(val) => Some(val.to_string()),
            SVGAttributes::Begin(val) => Some(val.to_string()),
            SVGAttributes::Bias(val) => Some(val.to_string()),
            SVGAttributes::By(val) => Some(val.to_string()),
            SVGAttributes::CalcMode(val) => Some(val.to_string()),
            SVGAttributes::CapHeight(val) => Some(val.to_string()),
            SVGAttributes::Clip(val) => Some(val.to_string()),
            SVGAttributes::ClipPath(val) => Some(val.to_string()),
            SVGAttributes::ClipPathUnits(val) => Some(val.to_string()),
            SVGAttributes::ClipRule(val) => Some(val.to_string()),
            SVGAttributes::ColorInterpolation(val) => Some(val.to_string()),
            SVGAttributes::ColorInterpolationFilters(val) => Some(val.to_string()),
            SVGAttributes::ColorProfile(val) => Some(val.to_string()),
            SVGAttributes::ColorRendering(val) => Some(val.to_string()),
            SVGAttributes::ContentScriptType(val) => Some(val.to_string()),
            SVGAttributes::ContentStyleType(val) => Some(val.to_string()),
            SVGAttributes::Cursor(val) => Some(val.to_string()),
            SVGAttributes::Cx(val) => Some(val.to_string()),
            SVGAttributes::Cy(val) => Some(val.to_string()),
            SVGAttributes::D(val) => Some(val.to_string()),
            SVGAttributes::Decelerate(val) => Some(val.to_string()),
            SVGAttributes::Descent(val) => Some(val.to_string()),
            SVGAttributes::DiffuseConstant(val) => Some(val.to_string()),
            SVGAttributes::Direction(val) => Some(val.to_string()),
            SVGAttributes::Display(val) => Some(val.to_string()),
            SVGAttributes::Divisor(val) => Some(val.to_string()),
            SVGAttributes::DominantBaseline(val) => Some(val.to_string()),
            SVGAttributes::Dur(val) => Some(val.to_string()),
            SVGAttributes::Dx(val) => Some(val.to_string()),
            SVGAttributes::Dy(val) => Some(val.to_string()),
            SVGAttributes::EdgeMode(val) => Some(val.to_string()),
            SVGAttributes::Elevation(val) => Some(val.to_string()),
            SVGAttributes::EnableBackground(val) => Some(val.to_string()),
            SVGAttributes::End(val) => Some(val.to_string()),
            SVGAttributes::Exponent(val) => Some(val.to_string()),
            SVGAttributes::ExternalResourcesRequired(val) => Some(val.to_string()),
            SVGAttributes::Fill(val) => Some(val.to_string()),
            SVGAttributes::FillOpacity(val) => Some(val.to_string()),
            SVGAttributes::FillRule(val) => Some(val.to_string()),
            SVGAttributes::Filter(val) => Some(val.to_string()),
            SVGAttributes::FilterRes(val) => Some(val.to_string()),
            SVGAttributes::FilterUnits(val) => Some(val.to_string()),
            SVGAttributes::FloodColor(val) => Some(val.to_string()),
            SVGAttributes::FloodOpacity(val) => Some(val.to_string()),
            SVGAttributes::Focusable(val) => Some(val.to_string()),
            SVGAttributes::FontFamily(val) => Some(val.to_string()),
            SVGAttributes::FontSize(val) => Some(val.to_string()),
            SVGAttributes::FontSizeAdjust(val) => Some(val.to_string()),
            SVGAttributes::FontStretch(val) => Some(val.to_string()),
            SVGAttributes::FontStyle(val) => Some(val.to_string()),
            SVGAttributes::FontVariant(val) => Some(val.to_string()),
            SVGAttributes::FontWeight(val) => Some(val.to_string()),
            SVGAttributes::Format(val) => Some(val.to_string()),
            SVGAttributes::Fr(val) => Some(val.to_string()),
            SVGAttributes::From(val) => Some(val.to_string()),
            SVGAttributes::Fx(val) => Some(val.to_string()),
            SVGAttributes::Fy(val) => Some(val.to_string()),
            SVGAttributes::G1(val) => Some(val.to_string()),
            SVGAttributes::G2(val) => Some(val.to_string()),
            SVGAttributes::GlyphName(val) => Some(val.to_string()),
            SVGAttributes::GlyphOrientationHorizontal(val) => Some(val.to_string()),
            SVGAttributes::GlyphOrientationVertical(val) => Some(val.to_string()),
            SVGAttributes::GlyphRef(val) => Some(val.to_string()),
            SVGAttributes::GradientTransform(val) => Some(val.to_string()),
            SVGAttributes::GradientUnits(val) => Some(val.to_string()),
            SVGAttributes::Hanging(val) => Some(val.to_string()),
            SVGAttributes::HorizAdvX(val) => Some(val.to_string()),
            SVGAttributes::HorizOriginX(val) => Some(val.to_string()),
            SVGAttributes::Href(val) => Some(val.to_string()),
            SVGAttributes::Ideographic(val) => Some(val.to_string()),
            SVGAttributes::ImageRendering(val) => Some(val.to_string()),
            SVGAttributes::In2(val) => Some(val.to_string()),
            SVGAttributes::In(val) => Some(val.to_string()),
            SVGAttributes::Intercept(val) => Some(val.to_string()),
            SVGAttributes::K1(val) => Some(val.to_string()),
            SVGAttributes::K2(val) => Some(val.to_string()),
            SVGAttributes::K3(val) => Some(val.to_string()),
            SVGAttributes::K4(val) => Some(val.to_string()),
            SVGAttributes::K(val) => Some(val.to_string()),
            SVGAttributes::KernelMatrix(val) => Some(val.to_string()),
            SVGAttributes::KernelUnitLength(val) => Some(val.to_string()),
            SVGAttributes::Kerning(val) => Some(val.to_string()),
            SVGAttributes::KeyPoints(val) => Some(val.to_string()),
            SVGAttributes::KeySplines(val) => Some(val.to_string()),
            SVGAttributes::KeyTimes(val) => Some(val.to_string()),
            SVGAttributes::LengthAdjust(val) => Some(val.to_string()),
            SVGAttributes::LetterSpacing(val) => Some(val.to_string()),
            SVGAttributes::LightingColor(val) => Some(val.to_string()),
            SVGAttributes::LimitingConeAngle(val) => Some(val.to_string()),
            SVGAttributes::Local(val) => Some(val.to_string()),
            SVGAttributes::MarkerEnd(val) => Some(val.to_string()),
            SVGAttributes::MarkerHeight(val) => Some(val.to_string()),
            SVGAttributes::MarkerMid(val) => Some(val.to_string()),
            SVGAttributes::MarkerStart(val) => Some(val.to_string()),
            SVGAttributes::MarkerUnits(val) => Some(val.to_string()),
            SVGAttributes::MarkerWidth(val) => Some(val.to_string()),
            SVGAttributes::Mask(val) => Some(val.to_string()),
            SVGAttributes::MaskContentUnits(val) => Some(val.to_string()),
            SVGAttributes::MaskUnits(val) => Some(val.to_string()),
            SVGAttributes::Mathematical(val) => Some(val.to_string()),
            SVGAttributes::Mode(val) => Some(val.to_string()),
            SVGAttributes::NumOctaves(val) => Some(val.to_string()),
            SVGAttributes::Offset(val) => Some(val.to_string()),
            SVGAttributes::Opacity(val) => Some(val.to_string()),
            SVGAttributes::Operator(val) => Some(val.to_string()),
            SVGAttributes::Order(val) => Some(val.to_string()),
            SVGAttributes::Orient(val) => Some(val.to_string()),
            SVGAttributes::Orientation(val) => Some(val.to_string()),
            SVGAttributes::Origin(val) => Some(val.to_string()),
            SVGAttributes::Overflow(val) => Some(val.to_string()),
            SVGAttributes::OverlinePosition(val) => Some(val.to_string()),
            SVGAttributes::PverlineThickness(val) => Some(val.to_string()),
            SVGAttributes::PaintOrder(val) => Some(val.to_string()),
            SVGAttributes::Panose1(val) => Some(val.to_string()),
            SVGAttributes::Path(val) => Some(val.to_string()),
            SVGAttributes::PathLength(val) => Some(val.to_string()),
            SVGAttributes::PatternContentUnits(val) => Some(val.to_string()),
            SVGAttributes::PatternTransform(val) => Some(val.to_string()),
            SVGAttributes::PatternUnits(val) => Some(val.to_string()),
            SVGAttributes::PointerEvents(val) => Some(val.to_string()),
            SVGAttributes::Points(val) => Some(val.to_string()),
            SVGAttributes::PointsAtX(val) => Some(val.to_string()),
            SVGAttributes::PointsAtY(val) => Some(val.to_string()),
            SVGAttributes::PointsAtZ(val) => Some(val.to_string()),
            SVGAttributes::PreserveAlpha(val) => Some(val.to_string()),
            SVGAttributes::OreserveAspectRatio(val) => Some(val.to_string()),
            SVGAttributes::PrimitiveUnits(val) => Some(val.to_string()),
            SVGAttributes::R(val) => Some(val.to_string()),
            SVGAttributes::Radius(val) => Some(val.to_string()),
            SVGAttributes::RefX(val) => Some(val.to_string()),
            SVGAttributes::RefY(val) => Some(val.to_string()),
            SVGAttributes::RenderingIntent(val) => Some(val.to_string()),
            SVGAttributes::RepeatCount(val) => Some(val.to_string()),
            SVGAttributes::RepeatDur(val) => Some(val.to_string()),
            SVGAttributes::RequiredExtensions(val) => Some(val.to_string()),
            SVGAttributes::RequiredFeatures(val) => Some(val.to_string()),
            SVGAttributes::Restart(val) => Some(val.to_string()),
            SVGAttributes::Result(val) => Some(val.to_string()),
            SVGAttributes::Rotate(val) => Some(val.to_string()),
            SVGAttributes::Rx(val) => Some(val.to_string()),
            SVGAttributes::Ry(val) => Some(val.to_string()),
            SVGAttributes::Scale(val) => Some(val.to_string()),
            SVGAttributes::Seed(val) => Some(val.to_string()),
            SVGAttributes::ShapeRendering(val) => Some(val.to_string()),
            SVGAttributes::Slope(val) => Some(val.to_string()),
            SVGAttributes::Spacing(val) => Some(val.to_string()),
            SVGAttributes::SpecularConstant(val) => Some(val.to_string()),
            SVGAttributes::SpecularExponent(val) => Some(val.to_string()),
            SVGAttributes::Speed(val) => Some(val.to_string()),
            SVGAttributes::SpreadMethod(val) => Some(val.to_string()),
            SVGAttributes::StartOffset(val) => Some(val.to_string()),
            SVGAttributes::StdDeviation(val) => Some(val.to_string()),
            SVGAttributes::Stemh(val) => Some(val.to_string()),
            SVGAttributes::Stemv(val) => Some(val.to_string()),
            SVGAttributes::StitchTiles(val) => Some(val.to_string()),
            SVGAttributes::StopColor(val) => Some(val.to_string()),
            SVGAttributes::StopOpacity(val) => Some(val.to_string()),
            SVGAttributes::StrikethroughPosition(val) => Some(val.to_string()),
            SVGAttributes::StrikethroughThickness(val) => Some(val.to_string()),
            SVGAttributes::String(val) => Some(val.to_string()),
            SVGAttributes::Stroke(val) => Some(val.to_string()),
            SVGAttributes::StrokeDasharray(val) => Some(val.to_string()),
            SVGAttributes::StrokeDashoffset(val) => Some(val.to_string()),
            SVGAttributes::StrokeLinecap(val) => Some(val.to_string()),
            SVGAttributes::StrokeLinejoin(val) => Some(val.to_string()),
            SVGAttributes::StrokeMiterlimit(val) => Some(val.to_string()),
            SVGAttributes::StrokeOpacity(val) => Some(val.to_string()),
            SVGAttributes::StrokeWidth(val) => Some(val.to_string()),
            SVGAttributes::SurfaceScale(val) => Some(val.to_string()),
            SVGAttributes::SystemLanguage(val) => Some(val.to_string()),
            SVGAttributes::TableValues(val) => Some(val.to_string()),
            SVGAttributes::TargetX(val) => Some(val.to_string()),
            SVGAttributes::TargetY(val) => Some(val.to_string()),
            SVGAttributes::TextAnchor(val) => Some(val.to_string()),
            SVGAttributes::TextDecoration(val) => Some(val.to_string()),
            SVGAttributes::TextLength(val) => Some(val.to_string()),
            SVGAttributes::TextRendering(val) => Some(val.to_string()),
            SVGAttributes::To(val) => Some(val.to_string()),
            SVGAttributes::Transform(val) => Some(val.to_string()),
            SVGAttributes::U1(val) => Some(val.to_string()),
            SVGAttributes::U2(val) => Some(val.to_string()),
            SVGAttributes::EnderlinePosition(val) => Some(val.to_string()),
            SVGAttributes::UnderlineThickness(val) => Some(val.to_string()),
            SVGAttributes::Unicode(val) => Some(val.to_string()),
            SVGAttributes::UnicodeBidi(val) => Some(val.to_string()),
            SVGAttributes::UnicodeRange(val) => Some(val.to_string()),
            SVGAttributes::UnitsPerEm(val) => Some(val.to_string()),
            SVGAttributes::VAlphabetic(val) => Some(val.to_string()),
            SVGAttributes::Values(val) => Some(val.to_string()),
            SVGAttributes::VectorEffect(val) => Some(val.to_string()),
            SVGAttributes::Version(val) => Some(val.to_string()),
            SVGAttributes::VertAdvY(val) => Some(val.to_string()),
            SVGAttributes::VertOriginX(val) => Some(val.to_string()),
            SVGAttributes::VertOriginY(val) => Some(val.to_string()),
            SVGAttributes::VHanging(val) => Some(val.to_string()),
            SVGAttributes::VIdeographic(val) => Some(val.to_string()),
            SVGAttributes::SiewBox(val) => Some(val.to_string()),
            SVGAttributes::ViewTarget(val) => Some(val.to_string()),
            SVGAttributes::Visibility(val) => Some(val.to_string()),
            SVGAttributes::VMathematical(val) => Some(val.to_string()),
            SVGAttributes::Widths(val) => Some(val.to_string()),
            SVGAttributes::WordSpacing(val) => Some(val.to_string()),
            SVGAttributes::WritingMode(val) => Some(val.to_string()),
            SVGAttributes::X1(val) => Some(val.to_string()),
            SVGAttributes::X2(val) => Some(val.to_string()),
            SVGAttributes::X(val) => Some(val.to_string()),
            SVGAttributes::XChannelSelector(val) => Some(val.to_string()),
            SVGAttributes::XHeight(val) => Some(val.to_string()),
            SVGAttributes::XlinkActuate(val) => Some(val.to_string()),
            SVGAttributes::XlinkArcrole(val) => Some(val.to_string()),
            SVGAttributes::XlinkHref(val) => Some(val.to_string()),
            SVGAttributes::XlinkRole(val) => Some(val.to_string()),
            SVGAttributes::XlinkShow(val) => Some(val.to_string()),
            SVGAttributes::XlinkTitle(val) => Some(val.to_string()),
            SVGAttributes::XlinkType(val) => Some(val.to_string()),
            SVGAttributes::XmlBase(val) => Some(val.to_string()),
            SVGAttributes::XmlLang(val) => Some(val.to_string()),
            SVGAttributes::Xmlns(val) => Some(val.to_string()),
            SVGAttributes::XmlnsXlink(val) => Some(val.to_string()),
            SVGAttributes::XmlSpace(val) => Some(val.to_string()),
            SVGAttributes::Y1(val) => Some(val.to_string()),
            SVGAttributes::Y2(val) => Some(val.to_string()),
            SVGAttributes::Y(val) => Some(val.to_string()),
            SVGAttributes::YChannelSelector(val) => Some(val.to_string()),
            SVGAttributes::Z(val) => Some(val.to_string()),
            SVGAttributes::ZoomAndPan(val) => Some(val.to_string()),
        }
    }
}

impl std::hash::Hash for SVGAttributes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

pub trait SvgAttributeReceiver {
    fn add_svg_attribute(&mut self, attribute: SVGAttributes) -> bool;

    fn remove_svg_attribute(&mut self, attribute: SVGAttributes) -> bool;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NumberOrString {
    Number(i64),
    String(String),
}

impl Display for NumberOrString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberOrString::Number(val) => write!(f, "{}", val),
            NumberOrString::String(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum CrossOrigin {
    Anonymous,
    UseCredentials,
    #[strum(serialize = "\"\"")] // TODO: add backslash once the interet comes back
    Blank,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Accumulate {
    None,
    Sum,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Additive {
    Replace,
    Sum,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum AlignmentBaseline {
    Auto,
    Baseline,
    BeforeEdge,
    TextBeforeEdge,
    Middle,
    Central,
    AfterEdge,
    TextAfterEdge,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
    Inherit,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum AllowReorder {
    No,
    Yes,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum ArabicForm {
    Initial,
    Medial,
    Terminal,
    Isolated,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum ClipRule {
    Nonzero,
    Evenodd,
    Inherit,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "camelCase")]
pub enum ClipPathUnits {
    UserSpaceOnUse,
    ObjectBoundingBox,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "camelCase")]
pub enum ColorInterpolationFilters {
    Auto,
    #[strum(serialize = "sRGB")]
    SRGB,
    #[strum(serialize = "linearRGB")]
    LinearRGB,
    Inherit,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum FillRule {
    Nonzero,
    Evenodd,
    Inherit,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Focusable {
    True,
    False,
    Auto,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum StrokeLinecap {
    Butt,
    Round,
    Square,
    Inherit,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum StrokeLinejoin {
    Miter,
    Round,
    Bevel,
    Inherit,
}

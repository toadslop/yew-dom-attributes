use strum::Display;

use crate::html_attributes::AriaRole;

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
    TabIndex(u64),
    CrossOrigin(CrossOrigin),
    // SVG Specific attributes
    AccentHeight(NumberOrString),
    Accumulate(Accumulate),
    Additive(Additive),
    AlignmentBaseline(AlignmentBaseline),
    #[strum(serialize = "allowReorder")]
    AllowReorder(AllowReorder),
    #[deprecated(
        note = "See details: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/alphabetic"
    )]
    Alphabetic(NumberOrString),
    Amplitude(NumberOrString),
    ArabicForm(ArabicForm),
    #[deprecated(
        note = "See details: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/ascent"
    )]
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
    ColorInterpolationFilters(ColorInt),
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

impl PartialEq for SVGAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NumberOrString {
    Number(u64),
    String(String),
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

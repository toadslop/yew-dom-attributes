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
    #[strum(serialize = "camelCase")]
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
    #[strum(serialize = "camelCase")]
    AttributeName(String),
    #[strum(serialize = "camelCase")]
    AttributeType(String),
    AutoReverse(bool),
    Azimuth(NumberOrString),
    #[strum(serialize = "camelCase")]
    BaseFrequency(NumberOrString),
    BaselineShift(NumberOrString),
    #[strum(serialize = "camelCase")]
    BaseProfile(NumberOrString),
    Bbox(NumberOrString),
    Begin(NumberOrString),
    Bias(NumberOrString),
    By(NumberOrString),
    CalcMode(NumberOrString),
    CapHeight(NumberOrString),
    Clip(NumberOrString),
    #[strum(serialize = "camelCase")]
    ClipPath(String),
    #[strum(serialize = "camelCase")]
    ClipPathUnits(ClipPathUnits),
    ClipRule(ClipRule),
    //         colorInterpolation?: number | string | undefined;
    //         colorInterpolationFilters?: "auto" | "sRGB" | "linearRGB" | "inherit" | undefined;
    //         colorProfile?: number | string | undefined;
    //         colorRendering?: number | string | undefined;
    //         contentScriptType?: number | string | undefined;
    //         contentStyleType?: number | string | undefined;
    //         cursor?: number | string | undefined;
    //         cx?: number | string | undefined;
    //         cy?: number | string | undefined;
    //         d?: string | undefined;
    //         decelerate?: number | string | undefined;
    //         descent?: number | string | undefined;
    //         diffuseConstant?: number | string | undefined;
    //         direction?: number | string | undefined;
    //         display?: number | string | undefined;
    //         divisor?: number | string | undefined;
    //         dominantBaseline?: number | string | undefined;
    //         dur?: number | string | undefined;
    //         dx?: number | string | undefined;
    //         dy?: number | string | undefined;
    //         edgeMode?: number | string | undefined;
    //         elevation?: number | string | undefined;
    //         enableBackground?: number | string | undefined;
    //         end?: number | string | undefined;
    //         exponent?: number | string | undefined;
    //         externalResourcesRequired?: Booleanish | undefined;
    //         fill?: string | undefined;
    //         fillOpacity?: number | string | undefined;
    //         fillRule?: "nonzero" | "evenodd" | "inherit" | undefined;
    //         filter?: string | undefined;
    //         filterRes?: number | string | undefined;
    //         filterUnits?: number | string | undefined;
    //         floodColor?: number | string | undefined;
    //         floodOpacity?: number | string | undefined;
    //         focusable?: Booleanish | "auto" | undefined;
    //         fontFamily?: string | undefined;
    //         fontSize?: number | string | undefined;
    //         fontSizeAdjust?: number | string | undefined;
    //         fontStretch?: number | string | undefined;
    //         fontStyle?: number | string | undefined;
    //         fontVariant?: number | string | undefined;
    //         fontWeight?: number | string | undefined;
    //         format?: number | string | undefined;
    //         fr?: number | string | undefined;
    //         from?: number | string | undefined;
    //         fx?: number | string | undefined;
    //         fy?: number | string | undefined;
    //         g1?: number | string | undefined;
    //         g2?: number | string | undefined;
    //         glyphName?: number | string | undefined;
    //         glyphOrientationHorizontal?: number | string | undefined;
    //         glyphOrientationVertical?: number | string | undefined;
    //         glyphRef?: number | string | undefined;
    //         gradientTransform?: string | undefined;
    //         gradientUnits?: string | undefined;
    //         hanging?: number | string | undefined;
    //         horizAdvX?: number | string | undefined;
    //         horizOriginX?: number | string | undefined;
    //         href?: string | undefined;
    //         ideographic?: number | string | undefined;
    //         imageRendering?: number | string | undefined;
    //         in2?: number | string | undefined;
    //         in?: string | undefined;
    //         intercept?: number | string | undefined;
    //         k1?: number | string | undefined;
    //         k2?: number | string | undefined;
    //         k3?: number | string | undefined;
    //         k4?: number | string | undefined;
    //         k?: number | string | undefined;
    //         kernelMatrix?: number | string | undefined;
    //         kernelUnitLength?: number | string | undefined;
    //         kerning?: number | string | undefined;
    //         keyPoints?: number | string | undefined;
    //         keySplines?: number | string | undefined;
    //         keyTimes?: number | string | undefined;
    //         lengthAdjust?: number | string | undefined;
    //         letterSpacing?: number | string | undefined;
    //         lightingColor?: number | string | undefined;
    //         limitingConeAngle?: number | string | undefined;
    //         local?: number | string | undefined;
    //         markerEnd?: string | undefined;
    //         markerHeight?: number | string | undefined;
    //         markerMid?: string | undefined;
    //         markerStart?: string | undefined;
    //         markerUnits?: number | string | undefined;
    //         markerWidth?: number | string | undefined;
    //         mask?: string | undefined;
    //         maskContentUnits?: number | string | undefined;
    //         maskUnits?: number | string | undefined;
    //         mathematical?: number | string | undefined;
    //         mode?: number | string | undefined;
    //         numOctaves?: number | string | undefined;
    //         offset?: number | string | undefined;
    //         opacity?: number | string | undefined;
    //         operator?: number | string | undefined;
    //         order?: number | string | undefined;
    //         orient?: number | string | undefined;
    //         orientation?: number | string | undefined;
    //         origin?: number | string | undefined;
    //         overflow?: number | string | undefined;
    //         overlinePosition?: number | string | undefined;
    //         overlineThickness?: number | string | undefined;
    //         paintOrder?: number | string | undefined;
    //         panose1?: number | string | undefined;
    //         path?: string | undefined;
    //         pathLength?: number | string | undefined;
    //         patternContentUnits?: string | undefined;
    //         patternTransform?: number | string | undefined;
    //         patternUnits?: string | undefined;
    //         pointerEvents?: number | string | undefined;
    //         points?: string | undefined;
    //         pointsAtX?: number | string | undefined;
    //         pointsAtY?: number | string | undefined;
    //         pointsAtZ?: number | string | undefined;
    //         preserveAlpha?: Booleanish | undefined;
    //         preserveAspectRatio?: string | undefined;
    //         primitiveUnits?: number | string | undefined;
    //         r?: number | string | undefined;
    //         radius?: number | string | undefined;
    //         refX?: number | string | undefined;
    //         refY?: number | string | undefined;
    //         renderingIntent?: number | string | undefined;
    //         repeatCount?: number | string | undefined;
    //         repeatDur?: number | string | undefined;
    //         requiredExtensions?: number | string | undefined;
    //         requiredFeatures?: number | string | undefined;
    //         restart?: number | string | undefined;
    //         result?: string | undefined;
    //         rotate?: number | string | undefined;
    //         rx?: number | string | undefined;
    //         ry?: number | string | undefined;
    //         scale?: number | string | undefined;
    //         seed?: number | string | undefined;
    //         shapeRendering?: number | string | undefined;
    //         slope?: number | string | undefined;
    //         spacing?: number | string | undefined;
    //         specularConstant?: number | string | undefined;
    //         specularExponent?: number | string | undefined;
    //         speed?: number | string | undefined;
    //         spreadMethod?: string | undefined;
    //         startOffset?: number | string | undefined;
    //         stdDeviation?: number | string | undefined;
    //         stemh?: number | string | undefined;
    //         stemv?: number | string | undefined;
    //         stitchTiles?: number | string | undefined;
    //         stopColor?: string | undefined;
    //         stopOpacity?: number | string | undefined;
    //         strikethroughPosition?: number | string | undefined;
    //         strikethroughThickness?: number | string | undefined;
    //         string?: number | string | undefined;
    //         stroke?: string | undefined;
    //         strokeDasharray?: string | number | undefined;
    //         strokeDashoffset?: string | number | undefined;
    //         strokeLinecap?: "butt" | "round" | "square" | "inherit" | undefined;
    //         strokeLinejoin?: "miter" | "round" | "bevel" | "inherit" | undefined;
    //         strokeMiterlimit?: number | string | undefined;
    //         strokeOpacity?: number | string | undefined;
    //         strokeWidth?: number | string | undefined;
    //         surfaceScale?: number | string | undefined;
    //         systemLanguage?: number | string | undefined;
    //         tableValues?: number | string | undefined;
    //         targetX?: number | string | undefined;
    //         targetY?: number | string | undefined;
    //         textAnchor?: string | undefined;
    //         textDecoration?: number | string | undefined;
    //         textLength?: number | string | undefined;
    //         textRendering?: number | string | undefined;
    //         to?: number | string | undefined;
    //         transform?: string | undefined;
    //         u1?: number | string | undefined;
    //         u2?: number | string | undefined;
    //         underlinePosition?: number | string | undefined;
    //         underlineThickness?: number | string | undefined;
    //         unicode?: number | string | undefined;
    //         unicodeBidi?: number | string | undefined;
    //         unicodeRange?: number | string | undefined;
    //         unitsPerEm?: number | string | undefined;
    //         vAlphabetic?: number | string | undefined;
    //         values?: string | undefined;
    //         vectorEffect?: number | string | undefined;
    //         version?: string | undefined;
    //         vertAdvY?: number | string | undefined;
    //         vertOriginX?: number | string | undefined;
    //         vertOriginY?: number | string | undefined;
    //         vHanging?: number | string | undefined;
    //         vIdeographic?: number | string | undefined;
    //         viewBox?: string | undefined;
    //         viewTarget?: number | string | undefined;
    //         visibility?: number | string | undefined;
    //         vMathematical?: number | string | undefined;
    //         widths?: number | string | undefined;
    //         wordSpacing?: number | string | undefined;
    //         writingMode?: number | string | undefined;
    //         x1?: number | string | undefined;
    //         x2?: number | string | undefined;
    //         x?: number | string | undefined;
    //         xChannelSelector?: string | undefined;
    //         xHeight?: number | string | undefined;
    //         xlinkActuate?: string | undefined;
    //         xlinkArcrole?: string | undefined;
    //         xlinkHref?: string | undefined;
    //         xlinkRole?: string | undefined;
    //         xlinkShow?: string | undefined;
    //         xlinkTitle?: string | undefined;
    //         xlinkType?: string | undefined;
    //         xmlBase?: string | undefined;
    //         xmlLang?: string | undefined;
    //         xmlns?: string | undefined;
    //         xmlnsXlink?: string | undefined;
    //         xmlSpace?: string | undefined;
    //         y1?: number | string | undefined;
    //         y2?: number | string | undefined;
    //         y?: number | string | undefined;
    //         yChannelSelector?: string | undefined;
    //         z?: number | string | undefined;
    //         zoomAndPan?: string | undefined;
    //     }
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
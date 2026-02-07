//! Typography tokens for theming

/// Semantic typography token keys for dynamic access
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum TypographyToken {
    // Font sizes
    TextXs,
    TextSm,
    TextBase,
    TextLg,
    TextXl,
    Text2xl,
    Text3xl,
    Text4xl,
    Text5xl,

    // Font weights (as numeric values)
    FontThin,
    FontLight,
    FontNormal,
    FontMedium,
    FontSemibold,
    FontBold,
    FontBlack,

    // Line heights (multipliers)
    LeadingNone,
    LeadingTight,
    LeadingSnug,
    LeadingNormal,
    LeadingRelaxed,
    LeadingLoose,

    // Letter spacing (em units)
    TrackingTighter,
    TrackingTight,
    TrackingNormal,
    TrackingWide,
    TrackingWider,
}

/// Font family definition
#[derive(Clone, Debug)]
pub struct FontFamily {
    /// Primary font name
    pub name: String,
    /// Fallback fonts
    pub fallbacks: Vec<String>,
}

impl FontFamily {
    pub fn new(name: impl Into<String>, fallbacks: Vec<&str>) -> Self {
        Self {
            name: name.into(),
            fallbacks: fallbacks.into_iter().map(String::from).collect(),
        }
    }

    /// System sans-serif font stack
    pub fn system_sans() -> Self {
        Self {
            name: "system-ui".into(),
            fallbacks: vec![
                "-apple-system".into(),
                "BlinkMacSystemFont".into(),
                "Segoe UI".into(),
                "Roboto".into(),
                "Oxygen".into(),
                "Ubuntu".into(),
                "sans-serif".into(),
            ],
        }
    }

    /// System monospace font stack
    pub fn system_mono() -> Self {
        Self {
            name: "ui-monospace".into(),
            fallbacks: vec![
                "SFMono-Regular".into(),
                "SF Mono".into(),
                "Menlo".into(),
                "Consolas".into(),
                "Liberation Mono".into(),
                "monospace".into(),
            ],
        }
    }

    /// System serif font stack
    pub fn system_serif() -> Self {
        Self {
            name: "ui-serif".into(),
            fallbacks: vec![
                "Georgia".into(),
                "Cambria".into(),
                "Times New Roman".into(),
                "Times".into(),
                "serif".into(),
            ],
        }
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        Self::system_sans()
    }
}

/// Font weight values
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
#[derive(Default)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    #[default]
    Normal = 400,
    Medium = 500,
    Semibold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl FontWeight {
    pub fn as_u16(self) -> u16 {
        self as u16
    }
}

/// Complete set of typography tokens
#[derive(Clone, Debug)]
pub struct TypographyTokens {
    // Font families
    pub font_sans: FontFamily,
    pub font_serif: FontFamily,
    pub font_mono: FontFamily,

    // Font sizes (in logical pixels)
    pub text_xs: f32,
    pub text_sm: f32,
    pub text_base: f32,
    pub text_lg: f32,
    pub text_xl: f32,
    pub text_2xl: f32,
    pub text_3xl: f32,
    pub text_4xl: f32,
    pub text_5xl: f32,

    // Font weights
    pub font_thin: FontWeight,
    pub font_light: FontWeight,
    pub font_normal: FontWeight,
    pub font_medium: FontWeight,
    pub font_semibold: FontWeight,
    pub font_bold: FontWeight,
    pub font_black: FontWeight,

    // Line heights (multipliers)
    pub leading_none: f32,
    pub leading_tight: f32,
    pub leading_snug: f32,
    pub leading_normal: f32,
    pub leading_relaxed: f32,
    pub leading_loose: f32,

    // Letter spacing (em units)
    pub tracking_tighter: f32,
    pub tracking_tight: f32,
    pub tracking_normal: f32,
    pub tracking_wide: f32,
    pub tracking_wider: f32,
}

impl TypographyTokens {
    /// Get a numeric token value by key
    pub fn get(&self, token: TypographyToken) -> f32 {
        match token {
            TypographyToken::TextXs => self.text_xs,
            TypographyToken::TextSm => self.text_sm,
            TypographyToken::TextBase => self.text_base,
            TypographyToken::TextLg => self.text_lg,
            TypographyToken::TextXl => self.text_xl,
            TypographyToken::Text2xl => self.text_2xl,
            TypographyToken::Text3xl => self.text_3xl,
            TypographyToken::Text4xl => self.text_4xl,
            TypographyToken::Text5xl => self.text_5xl,
            TypographyToken::FontThin => self.font_thin.as_u16() as f32,
            TypographyToken::FontLight => self.font_light.as_u16() as f32,
            TypographyToken::FontNormal => self.font_normal.as_u16() as f32,
            TypographyToken::FontMedium => self.font_medium.as_u16() as f32,
            TypographyToken::FontSemibold => self.font_semibold.as_u16() as f32,
            TypographyToken::FontBold => self.font_bold.as_u16() as f32,
            TypographyToken::FontBlack => self.font_black.as_u16() as f32,
            TypographyToken::LeadingNone => self.leading_none,
            TypographyToken::LeadingTight => self.leading_tight,
            TypographyToken::LeadingSnug => self.leading_snug,
            TypographyToken::LeadingNormal => self.leading_normal,
            TypographyToken::LeadingRelaxed => self.leading_relaxed,
            TypographyToken::LeadingLoose => self.leading_loose,
            TypographyToken::TrackingTighter => self.tracking_tighter,
            TypographyToken::TrackingTight => self.tracking_tight,
            TypographyToken::TrackingNormal => self.tracking_normal,
            TypographyToken::TrackingWide => self.tracking_wide,
            TypographyToken::TrackingWider => self.tracking_wider,
        }
    }
}

impl Default for TypographyTokens {
    fn default() -> Self {
        Self {
            font_sans: FontFamily::system_sans(),
            font_serif: FontFamily::system_serif(),
            font_mono: FontFamily::system_mono(),

            // Font sizes (Tailwind-inspired scale)
            text_xs: 12.0,
            text_sm: 14.0,
            text_base: 16.0,
            text_lg: 18.0,
            text_xl: 20.0,
            text_2xl: 24.0,
            text_3xl: 30.0,
            text_4xl: 36.0,
            text_5xl: 48.0,

            // Font weights
            font_thin: FontWeight::Thin,
            font_light: FontWeight::Light,
            font_normal: FontWeight::Normal,
            font_medium: FontWeight::Medium,
            font_semibold: FontWeight::Semibold,
            font_bold: FontWeight::Bold,
            font_black: FontWeight::Black,

            // Line heights
            leading_none: 1.0,
            leading_tight: 1.25,
            leading_snug: 1.375,
            leading_normal: 1.5,
            leading_relaxed: 1.625,
            leading_loose: 2.0,

            // Letter spacing (in em)
            tracking_tighter: -0.05,
            tracking_tight: -0.025,
            tracking_normal: 0.0,
            tracking_wide: 0.025,
            tracking_wider: 0.05,
        }
    }
}

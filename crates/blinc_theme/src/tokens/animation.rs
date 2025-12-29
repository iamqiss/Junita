//! Animation/transition tokens for theming

/// Semantic animation token keys for dynamic access
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum AnimationToken {
    // Durations
    DurationFastest,
    DurationFaster,
    DurationFast,
    DurationNormal,
    DurationSlow,
    DurationSlower,
    DurationSlowest,
}

/// Easing function type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    /// Custom cubic bezier (x1, y1, x2, y2)
    CubicBezier(f32, f32, f32, f32),
}

impl Easing {
    /// Evaluate the easing function at time t (0.0 to 1.0)
    pub fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Easing::CubicBezier(x1, y1, x2, y2) => {
                // Simplified cubic bezier - for full accuracy would need iterative solve
                cubic_bezier_approximate(t, *x1, *y1, *x2, *y2)
            }
        }
    }
}

impl Default for Easing {
    fn default() -> Self {
        Easing::EaseOut
    }
}

/// Approximate cubic bezier evaluation
fn cubic_bezier_approximate(t: f32, _x1: f32, y1: f32, _x2: f32, y2: f32) -> f32 {
    // Simple approximation - evaluate y at t directly
    // For accurate bezier, would need to solve for t given x
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;

    3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3
}

/// Complete set of animation tokens
#[derive(Clone, Debug)]
pub struct AnimationTokens {
    // Durations in milliseconds
    pub duration_fastest: u64,
    pub duration_faster: u64,
    pub duration_fast: u64,
    pub duration_normal: u64,
    pub duration_slow: u64,
    pub duration_slower: u64,
    pub duration_slowest: u64,

    // Easing functions
    pub ease_default: Easing,
    pub ease_in: Easing,
    pub ease_out: Easing,
    pub ease_in_out: Easing,
}

impl AnimationTokens {
    /// Get duration by token key (in milliseconds)
    pub fn get(&self, token: AnimationToken) -> u64 {
        match token {
            AnimationToken::DurationFastest => self.duration_fastest,
            AnimationToken::DurationFaster => self.duration_faster,
            AnimationToken::DurationFast => self.duration_fast,
            AnimationToken::DurationNormal => self.duration_normal,
            AnimationToken::DurationSlow => self.duration_slow,
            AnimationToken::DurationSlower => self.duration_slower,
            AnimationToken::DurationSlowest => self.duration_slowest,
        }
    }

    /// Get duration as seconds (f32)
    pub fn get_seconds(&self, token: AnimationToken) -> f32 {
        self.get(token) as f32 / 1000.0
    }
}

impl Default for AnimationTokens {
    fn default() -> Self {
        Self {
            duration_fastest: 75,
            duration_faster: 100,
            duration_fast: 150,
            duration_normal: 200,
            duration_slow: 300,
            duration_slower: 400,
            duration_slowest: 500,

            ease_default: Easing::EaseOut,
            ease_in: Easing::EaseIn,
            ease_out: Easing::EaseOut,
            ease_in_out: Easing::EaseInOut,
        }
    }
}

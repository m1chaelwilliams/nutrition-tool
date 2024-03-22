pub enum MeasureSystem {
    Imperial,
    Metric
}

impl Default for MeasureSystem {
    fn default() -> Self {
        Self::Imperial
    }
}

pub enum SexAtBirth {
    Male,
    Female
}

impl Default for SexAtBirth {
    fn default() -> Self {
        Self::Male
    }
}

pub enum ActivityLevel {
    Sedentary,
    LightlyActive,
    Active,
    VeryActive,
    SuperActive
}

impl Default for ActivityLevel {
    fn default() -> Self {
        Self::Active
    }
}
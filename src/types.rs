#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    #[allow(dead_code)]
    pub const ALL: [Theme; 2] = [Theme::Dark, Theme::Light];

    pub fn as_str(self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }

    pub fn parse(input: &str) -> Option<Theme> {
        match normalize(input).as_str() {
            "dark" | "d" | "night" => Some(Theme::Dark),
            "light" | "l" | "day" => Some(Theme::Light),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DevelopmentType {
    Backend,
    Frontend,
    Fullstack,
    DataScience,
    DevOps,
    Blockchain,
    MachineLearning,
    SystemsProgramming,
    GameDevelopment,
    Security,
}

impl DevelopmentType {
    #[allow(dead_code)]
    pub const ALL: [DevelopmentType; 10] = [
        DevelopmentType::Backend,
        DevelopmentType::Frontend,
        DevelopmentType::Fullstack,
        DevelopmentType::DataScience,
        DevelopmentType::DevOps,
        DevelopmentType::Blockchain,
        DevelopmentType::MachineLearning,
        DevelopmentType::SystemsProgramming,
        DevelopmentType::GameDevelopment,
        DevelopmentType::Security,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            DevelopmentType::Backend => "backend",
            DevelopmentType::Frontend => "frontend",
            DevelopmentType::Fullstack => "fullstack",
            DevelopmentType::DataScience => "data-science",
            DevelopmentType::DevOps => "devops",
            DevelopmentType::Blockchain => "blockchain",
            DevelopmentType::MachineLearning => "machine-learning",
            DevelopmentType::SystemsProgramming => "systems",
            DevelopmentType::GameDevelopment => "game-dev",
            DevelopmentType::Security => "security",
        }
    }

    pub fn parse(input: &str) -> Option<DevelopmentType> {
        match normalize(input).as_str() {
            "backend" | "back" => Some(DevelopmentType::Backend),
            "frontend" | "front" => Some(DevelopmentType::Frontend),
            "fullstack" | "full" => Some(DevelopmentType::Fullstack),
            "datascience" | "data" | "ds" => Some(DevelopmentType::DataScience),
            "devops" | "ops" => Some(DevelopmentType::DevOps),
            "blockchain" | "web3" => Some(DevelopmentType::Blockchain),
            "machinelearning" | "ml" | "ai" => Some(DevelopmentType::MachineLearning),
            "systems" | "systemsprogramming" | "rust" => Some(DevelopmentType::SystemsProgramming),
            "gamedev" | "gamedevelopment" | "game" => Some(DevelopmentType::GameDevelopment),
            "security" | "sec" => Some(DevelopmentType::Security),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum JargonLevel {
    Low,
    Medium,
    High,
    Extreme,
}

impl JargonLevel {
    #[allow(dead_code)]
    pub const ALL: [JargonLevel; 4] = [
        JargonLevel::Low,
        JargonLevel::Medium,
        JargonLevel::High,
        JargonLevel::Extreme,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            JargonLevel::Low => "low",
            JargonLevel::Medium => "medium",
            JargonLevel::High => "high",
            JargonLevel::Extreme => "extreme",
        }
    }

    pub fn parse(input: &str) -> Option<JargonLevel> {
        match normalize(input).as_str() {
            "low" | "l" => Some(JargonLevel::Low),
            "medium" | "med" | "m" => Some(JargonLevel::Medium),
            "high" | "h" => Some(JargonLevel::High),
            "extreme" | "x" => Some(JargonLevel::Extreme),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Complexity {
    Low,
    Medium,
    High,
    Extreme,
}

impl Complexity {
    #[allow(dead_code)]
    pub const ALL: [Complexity; 4] = [
        Complexity::Low,
        Complexity::Medium,
        Complexity::High,
        Complexity::Extreme,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Complexity::Low => "low",
            Complexity::Medium => "medium",
            Complexity::High => "high",
            Complexity::Extreme => "extreme",
        }
    }

    pub fn parse(input: &str) -> Option<Complexity> {
        match normalize(input).as_str() {
            "low" | "l" => Some(Complexity::Low),
            "medium" | "med" | "m" => Some(Complexity::Medium),
            "high" | "h" => Some(Complexity::High),
            "extreme" | "x" => Some(Complexity::Extreme),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub enum Speed {
    Slow,
    #[default]
    Normal,
    Fast,
    Turbo,
}

impl Speed {
    #[allow(dead_code)]
    pub const ALL: [Speed; 4] = [Speed::Slow, Speed::Normal, Speed::Fast, Speed::Turbo];

    pub fn as_str(self) -> &'static str {
        match self {
            Speed::Slow => "slow",
            Speed::Normal => "normal",
            Speed::Fast => "fast",
            Speed::Turbo => "turbo",
        }
    }

    pub fn parse(input: &str) -> Option<Speed> {
        match normalize(input).as_str() {
            "slow" | "s" => Some(Speed::Slow),
            "normal" | "norm" | "n" => Some(Speed::Normal),
            "fast" | "f" => Some(Speed::Fast),
            "turbo" | "t" => Some(Speed::Turbo),
            _ => None,
        }
    }

    /// Speed multiplier applied to base delays.
    pub fn delay_factor(self) -> f32 {
        match self {
            Speed::Slow => 1.8,
            Speed::Normal => 1.0,
            Speed::Fast => 0.5,
            Speed::Turbo => 0.2,
        }
    }
}

fn normalize(input: &str) -> String {
    input
        .trim()
        .to_ascii_lowercase()
        .chars()
        .filter(|c| *c != '-' && *c != '_' && *c != ' ')
        .collect()
}

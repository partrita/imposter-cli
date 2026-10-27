use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
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

    pub fn as_str(&self) -> &'static str {
        match self {
            DevelopmentType::Backend => "Backend",
            DevelopmentType::Frontend => "Frontend",
            DevelopmentType::Fullstack => "Fullstack",
            DevelopmentType::DataScience => "DataScience",
            DevelopmentType::DevOps => "DevOps",
            DevelopmentType::Blockchain => "Blockchain",
            DevelopmentType::MachineLearning => "MachineLearning",
            DevelopmentType::SystemsProgramming => "SystemsProgramming",
            DevelopmentType::GameDevelopment => "GameDevelopment",
            DevelopmentType::Security => "Security",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
pub enum JargonLevel {
    Low,
    Medium,
    High,
    Extreme,
}

impl JargonLevel {
    pub const ALL: [JargonLevel; 4] = [
        JargonLevel::Low,
        JargonLevel::Medium,
        JargonLevel::High,
        JargonLevel::Extreme,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            JargonLevel::Low => "Low",
            JargonLevel::Medium => "Medium",
            JargonLevel::High => "High",
            JargonLevel::Extreme => "Extreme",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
pub enum Complexity {
    Low,
    Medium,
    High,
    Extreme,
}

impl Complexity {
    pub const ALL: [Complexity; 4] = [
        Complexity::Low,
        Complexity::Medium,
        Complexity::High,
        Complexity::Extreme,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Complexity::Low => "Low",
            Complexity::Medium => "Medium",
            Complexity::High => "High",
            Complexity::Extreme => "Extreme",
        }
    }
}

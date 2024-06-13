use clap::{Args};

#[derive(Args)]
pub struct ContestArgs {
    /// Specify a contest (default)
    #[arg(short, long, group = "contest_type")]
    contest: bool,

    /// Specify a gym
    #[arg(short, long, group = "contest_type")]
    gym: bool,

    /// The contest ID
    pub contest_id: String,
}
impl ContestArgs {
    pub fn get_contest_type(&self) -> ContestType {
        if self.gym {
            return ContestType::Gym;
        } else if self.contest {
            return ContestType::Contest;
        }
        ContestType::default()
    }
}

#[derive(Default,Clone)]
pub enum ContestType {
    #[default]
    Contest,
    Gym,
}
impl std::fmt::Display for ContestType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ContestType::Contest => write!(f, "contest"),
            ContestType::Gym => write!(f, "gym"),
        }
    }
}

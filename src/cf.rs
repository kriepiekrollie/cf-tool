use std::path::PathBuf;
use same_file::is_same_file;

#[derive(Default, Clone, Debug)]
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
impl ContestType {
    pub fn from_path(path: &PathBuf, root: &PathBuf) -> Option<Self> {
        // Assume "path" is <root>/<contesttype>/<contestid>/
        let mut iter = path.iter().rev();
        let contest_type = iter.next()?.to_str()?;

        let test_path = root.join(&contest_type);

        match is_same_file(&path, &test_path) {
            Ok(true) => ContestType::from_str(contest_type),
            _ => None,
        }
    }
    pub fn get_path(&self, root: &PathBuf) -> PathBuf {
        root.join(format!("{}", self))
    }
    pub fn from_string(s: &String) -> Option<Self> {
        Self::from_str(s.as_str())
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "contest" => Some(ContestType::Contest),
            "gym" => Some(ContestType::Gym),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ContestInfo {
    typ: ContestType,
    id: String,
}
impl ContestInfo {
    pub fn from_path(path: &PathBuf, root: &PathBuf) -> Option<Self> {
        // Assume "path" is <root>/<contesttype>/<contestid>/
        let mut iter = path.iter().rev();
        let contest_id = iter.next()?.to_str()?.to_string();
        let contest_type = iter.next()?.to_str()?;

        let test_path = root.join(&contest_type).join(&contest_id);

        match is_same_file(&path, &test_path) {
            Ok(true) => 
                Some (Self {
                    typ: ContestType::from_str(contest_type)?,
                    id: contest_id,
                }),
            _ => None
        }
    }
    pub fn get_path(&self, root: &PathBuf) -> PathBuf {
        self.typ.get_path(&root).join(&self.id)
    }
}

#[derive(Debug)]
pub struct ProblemInfo {
    contest: ContestInfo,
    id: String,
}
impl ProblemInfo {
    pub fn from_path(path: &PathBuf, root: &PathBuf) -> Option<Self> {
        // Assume "path" is <root>/<contesttype>/<contestid>/<problemid>/
        let mut iter = path.iter().rev();
        let problem_id = iter.next()?.to_str()?.to_string();
        let contest_id = iter.next()?.to_str()?.to_string();
        let contest_type = iter.next()?.to_str()?;

        let test_path = root.join(&contest_type).join(&contest_id).join(&problem_id);

        match is_same_file(&path, &test_path) {
            Ok(true) => 
                Some (Self {
                    contest: ContestInfo {
                        typ: ContestType::from_str(contest_type)?,
                        id: contest_id,
                    },
                    id: problem_id,
                }),
            _ => None
        }
    }
    pub fn get_path(&self, root: &PathBuf) -> PathBuf {
        self.contest.get_path(&root).join(&self.id)
    }
}

pub enum Title {
    Unrated,
    Newbie,
    Pupil,
    Apprentice,
    Specialist,
    Expert,
    CandidateMaster,
    Master,
    InternationalMaster,
    Grandmaster,
    InternationalGrandmaster,
    LegendaryGrandmaster,
}

impl Title {
    pub fn from_rating(rating: i16) -> Self {
        if rating < 0 {
            Self::Unrated
        } else if rating < 1000 {
            Self::Newbie
        } else if rating < 1200 {
            Self::Pupil
        } else if rating < 1400 {
            Self::Apprentice
        } else if rating < 1600 {
            Self::Specialist
        } else if rating < 1800 {
            Self::Expert
        } else if rating < 2000 {
            Self::CandidateMaster
        } else if rating < 2200 {
            Self::Master
        } else if rating < 2400 {
            Self::InternationalMaster
        } else if rating < 2700 {
            Self::Grandmaster
        } else if rating < 3000 {
            Self::InternationalGrandmaster
        } else {
            Self::LegendaryGrandmaster
        }
    }
}

pub struct UserInfo {
    handle: String,
    rating: Option<u16>,
}

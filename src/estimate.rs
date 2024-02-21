use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Estimate {
    None,
    QuestionMark,
    Coffe,
    Zero,
    Half,
    One,
    Two,
    Three,
    Five,
    Eight,
    Thirteen,
    Twenty,
    Fourty,
    Hundred,
}

impl std::fmt::Display for Estimate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_string: Arc<str> = self.clone().into();
        write!(f, "{}", display_string)
    }
}

impl From<Estimate> for Arc<str> {
    fn from(estimate: Estimate) -> Arc<str> {
        match estimate {
            Estimate::None => "".into(),
            Estimate::QuestionMark => "?".into(),
            Estimate::Coffe => "☕️".into(),
            Estimate::Zero => "0".into(),
            Estimate::Half => "0.5".into(),
            Estimate::One => "1".into(),
            Estimate::Two => "2".into(),
            Estimate::Three => "3".into(),
            Estimate::Five => "5".into(),
            Estimate::Eight => "8".into(),
            Estimate::Thirteen => "13".into(),
            Estimate::Twenty => "20".into(),
            Estimate::Fourty => "40".into(),
            Estimate::Hundred => "100".into(),
        }
    }
}

impl From<Estimate> for i32 {
    fn from(estimate: Estimate) -> i32 {
        match estimate {
            Estimate::None => -1,
            Estimate::QuestionMark => 0,
            Estimate::Coffe => 1,
            Estimate::Zero => 2,
            Estimate::Half => 3,
            Estimate::One => 4,
            Estimate::Two => 5,
            Estimate::Three => 6,
            Estimate::Five => 7,
            Estimate::Eight => 8,
            Estimate::Thirteen => 9,
            Estimate::Twenty => 10,
            Estimate::Fourty => 11,
            Estimate::Hundred => 12,
        }
    }
}

use simapi_sys as ffi;

/// Current simulator/session state reported by simapi.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStatus {
    /// No simulator session is available.
    Off,
    /// A simulator is running, but not in active gameplay.
    Menu,
    /// Telemetry is active and gameplay is in progress.
    ActivePlay,
    /// A value not currently known by this crate.
    Unknown(u32),
}

impl SessionStatus {
    pub(crate) fn from_raw(value: u32) -> Self {
        match value {
            x if x == ffi::SIMAPI_STATUS_SIMAPI_STATUS_OFF => Self::Off,
            x if x == ffi::SIMAPI_STATUS_SIMAPI_STATUS_MENU => Self::Menu,
            x if x == ffi::SIMAPI_STATUS_SIMAPI_STATUS_ACTIVEPLAY => Self::ActivePlay,
            other => Self::Unknown(other),
        }
    }

    /// Returns `true` when telemetry is coming from an active driving session.
    pub fn is_active(self) -> bool {
        matches!(self, Self::ActivePlay)
    }
}

/// Gear position reported by simapi.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gear {
    /// Reverse gear.
    Reverse,
    /// Neutral gear.
    Neutral,
    /// A forward gear, starting at 1.
    Forward(u8),
    /// A value not currently known by this crate.
    Unknown(u32),
}

impl Gear {
    pub(crate) fn from_raw(value: u32) -> Self {
        match value {
            x if x == ffi::SIMAPI_GEAR_SIMAPI_GEAR_REVERSE => Self::Reverse,
            x if x == ffi::SIMAPI_GEAR_SIMAPI_GEAR_NEUTRAL => Self::Neutral,
            x if x == ffi::SIMAPI_GEAR_SIMAPI_GEAR_FIRST => Self::Forward(1),
            x if x == ffi::SIMAPI_GEAR_SIMAPI_GEAR_SECOND => Self::Forward(2),
            x if x == ffi::SIMAPI_GEAR_SIMAPI_GEAR_THIRD => Self::Forward(3),
            x if x == ffi::SIMAPI_GEAR_SIMAPI_GEAR_FOURTH => Self::Forward(4),
            x if x == ffi::SIMAPI_GEAR_SIMAPI_GEAR_FIFTH => Self::Forward(5),
            x if x == ffi::SIMAPI_GEAR_SIMAPI_GEAR_SIXTH => Self::Forward(6),
            x if x == ffi::SIMAPI_GEAR_SIMAPI_GEAR_SEVENTH => Self::Forward(7),
            x if x == ffi::SIMAPI_GEAR_SIMAPI_GEAR_EIGHT => Self::Forward(8),
            other => Self::Unknown(other),
        }
    }

    /// Returns a short display form such as `R`, `N`, or `3`.
    pub fn display(self) -> String {
        match self {
            Self::Reverse => "R".to_string(),
            Self::Neutral => "N".to_string(),
            Self::Forward(n) => n.to_string(),
            Self::Unknown(_) => "?".to_string(),
        }
    }
}

/// Race/session flag state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaceFlag {
    /// Green flag.
    Green,
    /// Yellow flag.
    Yellow,
    /// Red flag.
    Red,
    /// Chequered flag.
    Chequered,
    /// Blue flag.
    Blue,
    /// White flag.
    White,
    /// Black flag.
    Black,
    /// Black and white flag.
    BlackWhite,
    /// Black and orange flag.
    BlackOrange,
    /// Orange flag.
    Orange,
    /// A value not currently known by this crate.
    Unknown(u8),
}

impl RaceFlag {
    pub(crate) fn from_raw(value: u8) -> Self {
        match value as u32 {
            x if x == ffi::SIMAPI_FLAG_SIMAPI_FLAG_GREEN => Self::Green,
            x if x == ffi::SIMAPI_FLAG_SIMAPI_FLAG_YELLOW => Self::Yellow,
            x if x == ffi::SIMAPI_FLAG_SIMAPI_FLAG_RED => Self::Red,
            x if x == ffi::SIMAPI_FLAG_SIMAPI_FLAG_CHEQUERED => Self::Chequered,
            x if x == ffi::SIMAPI_FLAG_SIMAPI_FLAG_BLUE => Self::Blue,
            x if x == ffi::SIMAPI_FLAG_SIMAPI_FLAG_WHITE => Self::White,
            x if x == ffi::SIMAPI_FLAG_SIMAPI_FLAG_BLACK => Self::Black,
            x if x == ffi::SIMAPI_FLAG_SIMAPI_FLAG_BLACK_WHITE => Self::BlackWhite,
            x if x == ffi::SIMAPI_FLAG_SIMAPI_FLAG_BLACK_ORANGE => Self::BlackOrange,
            x if x == ffi::SIMAPI_FLAG_SIMAPI_FLAG_ORANGE => Self::Orange,
            _ => Self::Unknown(value),
        }
    }
}

/// Simple lap time representation used by simapi.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LapTime {
    /// Hour component.
    pub hours: u32,
    /// Minute component.
    pub minutes: u32,
    /// Second component.
    pub seconds: u32,
    /// Fractional component reported by simapi.
    pub fraction: u32,
}

impl LapTime {
    pub(crate) fn from_raw(raw: ffi::LapTime) -> Self {
        Self {
            hours: raw.hours,
            minutes: raw.minutes,
            seconds: raw.seconds,
            fraction: raw.fraction,
        }
    }
}

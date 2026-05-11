use crate::{
    error::{Error, Result},
    types::{Gear, LapTime, RaceFlag, SessionStatus},
};

use simapi_sys as ffi;

const MAX_CARS: usize = ffi::MAXCARS as usize;
const PROXIMITY_CARS: usize = ffi::PROXCARS as usize;

/// Per-car data taken from the upstream `CarData` entry.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct CarData {
    /// World X position.
    pub x_pos: f64,
    /// World Y position.
    pub y_pos: f64,
    /// World Z position.
    pub z_pos: f64,
    /// Car spline position around the track.
    pub car_spline: f64,
    /// Current speed.
    pub speed: f64,
    /// Race position.
    pub position: u32,
    /// Current lap number.
    pub lap: u32,
    /// Track position.
    pub track_position: u32,
    /// Last lap time.
    pub last_lap: LapTime,
    /// Best lap time.
    pub best_lap: LapTime,
    /// Whether the car is in the pit.
    pub in_pit: bool,
    /// Whether the car is in the pit lane.
    pub in_pit_lane: bool,
    /// Whether the car is in the garage.
    pub in_garage: bool,
    /// Whether the car is at the pit entrance.
    pub in_pit_entrance: bool,
    /// Whether the car is at the pit exit.
    pub in_pit_exit: bool,
    /// Whether the car is stationary in the pit.
    pub in_pit_stopped: bool,
    /// Driver name.
    pub driver: String,
    /// Car name.
    pub car: String,
}

impl CarData {
    fn from_raw(raw: &ffi::CarData) -> Self {
        Self {
            x_pos: raw.xpos,
            y_pos: raw.ypos,
            z_pos: raw.zpos,
            car_spline: raw.carspline,
            speed: raw.speed,
            position: raw.pos,
            lap: raw.lap,
            track_position: raw.trackpos,
            last_lap: LapTime::from_raw(raw.lastlap),
            best_lap: LapTime::from_raw(raw.bestlap),
            in_pit: raw.inpit,
            in_pit_lane: raw.inpitlane,
            in_garage: raw.ingarage,
            in_pit_entrance: raw.inpitentrance,
            in_pit_exit: raw.inpitexit,
            in_pit_stopped: raw.inpitstopped,
            driver: c_char_array_to_string(&raw.driver),
            car: c_char_array_to_string(&raw.car),
        }
    }
}

/// Nearby car information from the upstream proximity array.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProximityData {
    /// Distance to the nearby car.
    pub radius: f64,
    /// Angle to the nearby car.
    pub theta: f64,
    /// Lap number for the nearby car.
    pub lap: u32,
}

impl ProximityData {
    fn from_raw(raw: &ffi::ProximityData) -> Self {
        Self {
            radius: raw.radius,
            theta: raw.theta,
            lap: raw.lap,
        }
    }
}

/// Full safe snapshot of the upstream `SimData` struct.
///
/// Field names stay close to the original C API on purpose so it is easy to
/// compare values with upstream simapi documentation, bindings, and tools.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct TelemetrySnapshot {
    /// Current millisecond tick.
    pub mtick: u64,
    /// Previous millisecond tick.
    pub prev_mtick: u64,

    /// Whether a simulator is currently on.
    pub sim_on: bool,
    /// simapi version written into the snapshot.
    pub simapi_version: u8,

    /// Session status.
    pub status: SessionStatus,
    /// Current gear as a Rust enum.
    pub gear: Gear,
    /// Vehicle speed.
    pub velocity: u32,
    /// Current engine RPM.
    pub rpms: u32,
    pub pulses: u32,
    pub max_rpm: u32,
    pub idle_rpm: u32,
    pub max_gears: u32,
    pub altitude: u32,
    pub lap: u32,
    pub position: u32,
    pub num_laps: u32,
    pub player_laps: u32,
    pub num_cars: u32,
    pub gearc: String,

    pub x_velocity: f64,
    pub y_velocity: f64,
    pub z_velocity: f64,
    pub world_x_velocity: f64,
    pub world_y_velocity: f64,
    pub world_z_velocity: f64,

    pub gas: f64,
    pub brake: f64,
    pub fuel: f64,
    pub fuel_capacity: f64,
    pub clutch: f64,
    pub steer: f64,
    pub handbrake: f64,

    pub turbo_boost: f64,
    pub turbo_boost_percent: f64,
    pub max_turbo: f64,

    pub abs: f64,
    pub brake_bias: f64,
    pub tyre_rps: [f64; 4],
    pub tyre_diameter: [f64; 4],
    pub tyre_slip_ratio: [f64; 4],
    pub tyre_slip_angle: [f64; 4],
    pub distance: f64,

    pub heading: f64,
    pub pitch: f64,
    pub roll: f64,
    pub world_pos_x: f64,
    pub world_pos_y: f64,
    pub world_pos_z: f64,

    pub brake_temps: [f64; 4],
    pub tyre_wear: [f64; 4],
    pub tyre_temps: [f64; 4],
    pub tyre_pressures: [f64; 4],
    pub tyre_contact_0: [f64; 4],
    pub tyre_contact_1: [f64; 4],
    pub tyre_contact_2: [f64; 4],

    pub air_density: f64,
    pub air_temp: f64,
    pub track_temp: f64,

    pub suspension: [f64; 4],
    pub suspension_velocity: [f64; 4],

    pub track_distance_around: f64,
    pub player_spline: f64,
    pub track_spline: f64,
    pub player_track_position: u32,
    pub track_samples: u32,

    pub current_lap: LapTime,
    pub last_lap: LapTime,
    pub best_lap: LapTime,
    pub current_lap_in_seconds: u32,
    pub last_lap_in_seconds: u32,
    pub time: u32,
    pub session_time: LapTime,
    pub session: u8,
    pub sector_index: u8,
    pub sector1_time: f64,
    pub sector2_time: f64,
    pub last_sector_in_ms: u32,
    pub course_flag: RaceFlag,
    pub player_flag: RaceFlag,
    pub lap_is_valid: bool,

    pub car: String,
    pub track: String,
    pub driver: String,
    pub tyre_compound: String,

    pub cars: [CarData; MAX_CARS],
    pub proximity: [ProximityData; PROXIMITY_CARS],

    /// Upstream simulator API identifier.
    pub simapi: u8,
    /// Upstream simulator executable identifier.
    pub simexe: u64,
}

impl TelemetrySnapshot {
    pub(crate) fn from_raw(raw: &ffi::SimData) -> Result<Self> {
        let expected_version = ffi::SIMAPI_VERSION as u8;
        let actual_version = raw.simapiversion;

        if actual_version != 0 && actual_version != expected_version {
            return Err(Error::VersionMismatch {
                expected: expected_version,
                actual: actual_version,
            });
        }

        Ok(Self {
            mtick: raw.mtick,
            prev_mtick: raw.prev_mtick,
            sim_on: raw.simon,
            simapi_version: raw.simapiversion,
            status: SessionStatus::from_raw(raw.simstatus),
            gear: Gear::from_raw(raw.gear),
            velocity: raw.velocity,
            rpms: raw.rpms,
            pulses: raw.pulses,
            max_rpm: raw.maxrpm,
            idle_rpm: raw.idlerpm,
            max_gears: raw.maxgears,
            altitude: raw.altitude,
            lap: raw.lap,
            position: raw.position,
            num_laps: raw.numlaps,
            player_laps: raw.playerlaps,
            num_cars: raw.numcars,
            gearc: c_char_array_to_string(&raw.gearc),
            x_velocity: raw.Xvelocity,
            y_velocity: raw.Yvelocity,
            z_velocity: raw.Zvelocity,
            world_x_velocity: raw.worldXvelocity,
            world_y_velocity: raw.worldYvelocity,
            world_z_velocity: raw.worldZvelocity,
            gas: raw.gas,
            brake: raw.brake,
            fuel: raw.fuel,
            fuel_capacity: raw.fuelcapacity,
            clutch: raw.clutch,
            steer: raw.steer,
            handbrake: raw.handbrake,
            turbo_boost: raw.turboboost,
            turbo_boost_percent: raw.turboboostperct,
            max_turbo: raw.maxturbo,
            abs: raw.abs,
            brake_bias: raw.brakebias,
            tyre_rps: raw.tyreRPS,
            tyre_diameter: raw.tyrediameter,
            tyre_slip_ratio: raw.tyreslipratio,
            tyre_slip_angle: raw.tyreslipangle,
            distance: raw.distance,
            heading: raw.heading,
            pitch: raw.pitch,
            roll: raw.roll,
            world_pos_x: raw.worldposx,
            world_pos_y: raw.worldposy,
            world_pos_z: raw.worldposz,
            brake_temps: raw.braketemp,
            tyre_wear: raw.tyrewear,
            tyre_temps: raw.tyretemp,
            tyre_pressures: raw.tyrepressure,
            tyre_contact_0: raw.tyrecontact0,
            tyre_contact_1: raw.tyrecontact1,
            tyre_contact_2: raw.tyrecontact2,
            air_density: raw.airdensity,
            air_temp: raw.airtemp,
            track_temp: raw.tracktemp,
            suspension: raw.suspension,
            suspension_velocity: raw.suspvelocity,
            track_distance_around: raw.trackdistancearound,
            player_spline: raw.playerspline,
            track_spline: raw.trackspline,
            player_track_position: raw.playertrackpos,
            track_samples: raw.tracksamples,
            current_lap: LapTime::from_raw(raw.currentlap),
            last_lap: LapTime::from_raw(raw.lastlap),
            best_lap: LapTime::from_raw(raw.bestlap),
            current_lap_in_seconds: raw.currentlapinseconds,
            last_lap_in_seconds: raw.lastlapinseconds,
            time: raw.time,
            session_time: LapTime::from_raw(raw.sessiontime),
            session: raw.session,
            sector_index: raw.sectorindex,
            sector1_time: raw.sector1time,
            sector2_time: raw.sector2time,
            last_sector_in_ms: raw.lastsectorinms,
            course_flag: RaceFlag::from_raw(raw.courseflag),
            player_flag: RaceFlag::from_raw(raw.playerflag),
            lap_is_valid: raw.lapisvalid,
            car: c_char_array_to_string(&raw.car),
            track: c_char_array_to_string(&raw.track),
            driver: c_char_array_to_string(&raw.driver),
            tyre_compound: c_char_array_to_string(&raw.tyrecompound),
            cars: std::array::from_fn(|index| CarData::from_raw(&raw.cars[index])),
            proximity: std::array::from_fn(|index| ProximityData::from_raw(&raw.pd[index])),
            simapi: raw.simapi,
            simexe: raw.simexe,
        })
    }
}

fn c_char_array_to_string<const N: usize>(array: &[std::os::raw::c_char; N]) -> String {
    let bytes = unsafe {
        // SAFETY:
        // The char array is valid for N bytes and may be viewed as raw bytes.
        std::slice::from_raw_parts(array.as_ptr().cast::<u8>(), N)
    };
    let len = bytes.iter().position(|byte| *byte == 0).unwrap_or(N);

    String::from_utf8_lossy(&bytes[..len]).into_owned()
}

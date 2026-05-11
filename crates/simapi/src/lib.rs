//! Safe Rust bindings for [`simapi`](https://github.com/Spacefreak18/simapi).
//!
//! This crate wraps the low-level `simapi-sys` bindings with a small, idiomatic
//! API centered around [`Client`] and [`TelemetrySnapshot`].
//!
//! # Example
//!
//! ```no_run
//! fn main() -> simapi::Result<()> {
//!     let mut client = simapi::Client::open()?;
//!     let snapshot = client.snapshot()?;
//!
//!     println!(
//!         "sim_on={} speed={} rpm={} car={}",
//!         snapshot.sim_on,
//!         snapshot.velocity,
//!         snapshot.rpms,
//!         snapshot.car,
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
#![cfg_attr(not(target_os = "linux"), allow(unused))]

#[cfg(not(target_os = "linux"))]
compile_error!("simapi currently supports Linux only");

mod error;
mod snapshot;
mod types;

pub use error::{Error, Result};
pub use snapshot::{CarData, ProximityData, TelemetrySnapshot};
pub use types::{Gear, LapTime, RaceFlag, SessionStatus};

use std::{
    os::raw::c_int,
    ptr::{self, NonNull},
};

use simapi_sys as ffi;

/// Connection handle used to discover the running simulator and read telemetry.
///
/// `Client` follows the same discovery flow used by the upstream C tools: it
/// creates a `SimMap`, asks simapi which backend is active, and refreshes the
/// current mapping each time [`snapshot`](Self::snapshot) is called.
pub struct Client {
    simmap: NonNull<ffi::SimMap>,
    simdata: Box<ffi::SimData>,
    mapapi: Option<ffi::SimulatorAPI>,
    use_udp: bool,
}

impl Client {
    /// Creates a new telemetry client.
    ///
    /// This does not force a connection to a specific simulator up front. The
    /// actual backend is discovered lazily on the first calls to
    /// [`snapshot`](Self::snapshot).
    pub fn open() -> Result<Self> {
        let simmap = unsafe {
            // SAFETY:
            // FFI call creates an opaque SimMap owned by the C library.
            ffi::simapi_simmap_create()
        };

        let simmap = NonNull::new(simmap).ok_or(Error::CreateMap)?;
        let simdata = Box::<ffi::SimData>::default();

        Ok(Self {
            simmap,
            simdata,
            mapapi: None,
            use_udp: false,
        })
    }

    /// Reads the latest telemetry snapshot.
    ///
    /// If no simulator is active yet, the snapshot will still be returned, but
    /// fields such as [`TelemetrySnapshot::sim_on`] and
    /// [`TelemetrySnapshot::status`] will reflect that no active session is
    /// available.
    pub fn snapshot(&mut self) -> Result<TelemetrySnapshot> {
        if self.mapapi.is_none() || !self.simdata.simon || self.simdata.simstatus <= 1 {
            self.refresh_sim_state();
        }

        if let Some(mapapi) = self.mapapi {
            let rc = unsafe {
                // SAFETY:
                // simmap and simdata are valid, and mapapi/use_udp come from the
                // upstream discovery flow used by simmonitor.
                ffi::simapi_datamap(
                    &mut *self.simdata,
                    self.simmap.as_ptr(),
                    mapapi,
                    self.use_udp,
                    ptr::null_mut(),
                )
            };

            if rc != 0 {
                return Err(Error::ReadTelemetry(rc));
            }
        }

        if !self.simdata.simon || self.simdata.simstatus <= 1 {
            self.mapapi = None;
            self.use_udp = false;
        }

        TelemetrySnapshot::from_raw(&self.simdata)
    }

    /// Returns the last raw `SimData` value owned by the client.
    ///
    /// Most users should prefer [`snapshot`](Self::snapshot), which converts
    /// the raw telemetry into safe Rust types.
    pub fn raw(&self) -> &ffi::SimData {
        &self.simdata
    }

    /// Returns the simapi version reported by the current snapshot.
    pub fn simapi_version(&self) -> u8 {
        self.raw().simapiversion
    }

    /// Returns whether simapi currently reports a running simulator.
    pub fn is_sim_on(&self) -> bool {
        self.raw().simon
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            // SAFETY:
            // This matches the consumer cleanup path available from simapi.
            let _ = ffi::simapi_sim_clear(&mut *self.simdata, self.simmap.as_ptr(), false);
        }

        unsafe { free_simmap(self.simmap) };
    }
}

impl Client {
    fn refresh_sim_state(&mut self) {
        let siminfo = unsafe {
            // SAFETY:
            // This mirrors simmonitor's discovery loop: inspect current sim
            // state, initialize the correct mapping, and return the mapapi to
            // use for subsequent data refreshes.
            ffi::simapi_get_sim(
                &mut *self.simdata,
                self.simmap.as_ptr(),
                false,
                Some(noop_setup_udp),
                false,
            )
        };

        if siminfo.isSimOn && self.simdata.simstatus >= 2 {
            self.mapapi = Some(siminfo.mapapi);
            self.use_udp = siminfo.SimUsesUDP;
        } else {
            self.mapapi = None;
            self.use_udp = false;
        }
    }
}

unsafe extern "C" fn noop_setup_udp(_port: c_int) -> c_int {
    0
}

unsafe fn free_simmap(simmap: NonNull<ffi::SimMap>) {
    // SAFETY:
    // simmap comes from simapi_simmap_create, which allocates it with malloc.
    unsafe {
        libc::free(simmap.as_ptr().cast());
    }
}

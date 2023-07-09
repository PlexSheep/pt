//! # monitor your network uptime
//!
//! This method offers a way to monitor your networks/hosts uptime. This is achieved by making
//! HTTPS requests to a given list of
//!
//! Warning: This module is not unit tested.

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
////////////////////////////////////////////////////////////////////////////////////////////////////
// we want Debug everywhere.
#![warn(missing_debug_implementations)]
////////////////////////////////////////////////////////////////////////////////////////////////////
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

use std::{fmt, str::FromStr, time::Duration};

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
// we want the log macros in any case
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use reqwest::{self, Url};

use humantime::{format_duration, format_rfc3339};
use std::time::SystemTime;

use pyo3::prelude::*;

use signal_hook::consts::SIGINT;

use crate::divider;

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////
/// urls used for checking by default
pub const DEFAULT_CHECK_URLS: &'static [&'static str] =
    &["https://www.cscherr.de", "https://www.cloudflare.com"];

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////
/// ## Describes an uptime status
///
/// [`UptimeStatus`] describes the result of an uptime check.
#[pyclass]
pub struct UptimeStatus {
    /// true if the [`UptimeStatus`] is considered successful
    pub success: bool,
    /// the percentage of reachable urls out of the total urls
    pub success_ratio: u8,
    /// the percentage of reachable urls out of the total urls that need to be reachable in order
    /// for this [`UptimeStatus`] to be considered a success.
    pub success_ratio_target: u8,
    /// the number of reachable [`urls`](UptimeStatus::urls)
    pub reachable: usize,
    /// which urls to check in [`check()`](UptimeStatus::check)
    pub urls: Vec<Url>,
}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
/// Main implementation
impl UptimeStatus {
    /// ## create a new `UptimeStatus` and perform it's check
    pub fn new(success_ratio_target: u8, url_strs: &Vec<String>) -> Self {
        assert!(success_ratio_target <= 100);
        let mut status = UptimeStatus {
            success: false,
            success_ratio: 0,
            success_ratio_target,
            reachable: 0,
            urls: Vec::new(),
        };
        for s in url_strs {
            let url = reqwest::Url::from_str(&s);
            if url.is_ok() {
                status.urls.push(url.unwrap());
            } else {
                warn!("Invalid URL: '{}", s);
            }
        }
        status.urls.dedup();

        status.check();

        return status;
    }

    /// ## check for success with the given urls
    ///
    /// Makes the actual https requests and updates fields accordingly.
    ///
    /// Note: Blocking execution for all requests, timeout is set to
    /// [REQUEST_TIMEOUT](crate::networking::REQUEST_TIMEOUT).
    pub fn check(&mut self) {
        self.reachable = 0;
        self.urls.iter().for_each(|url| {
            let client = reqwest::blocking::Client::builder()
                .timeout(Duration::from_millis(crate::networking::REQUEST_TIMEOUT))
                .build()
                .expect("could not build a client for https requests");
            let response = client.get(url.clone()).send();
            if response.is_ok() {
                self.reachable += 1
            }
        });
        self.calc_success();
    }

    /// ## calculate the success based on the `reachable` and `total`
    ///
    /// Calculates the ratio of [`reachable`](UptimeStatus::reachable) /
    /// (length of [urls](UptimeStatus::urls)).
    ///
    /// Calculates a [`success_ratio`](UptimeStatus::success_ratio) (as [u8]) from that,
    /// by multiplying with 100, then flooring.
    ///
    /// If the [`success_ratio`](UptimeStatus::success_ratio) is greater than or equal to the
    /// [`success_ratio_target`](UptimeStatus::success_ratio_target), the [`UptimeStatus`] will be
    /// considered a success.
    ///
    /// In the special case that no URLs to check for have been provided, the check will be
    /// considered a success, but the [`success_ratio`](UptimeStatus::success_ratio) will be `0`.
    ///
    /// Note: does not check for networking, use [`check()`](UptimeStatus::check) for that.
    pub fn calc_success(&mut self) {
        // if no urls need to be checked, success without checking
        if self.urls.len() == 0 {
            self.success = true;
            self.success_ratio = 0;
            return;
        }
        let ratio: f32 = (self.reachable as f32) / (self.urls.len() as f32) * 100f32;
        trace!("calculated success_ratio: {}", ratio);
        self.success_ratio = ratio.floor() as u8;
        self.success = self.success_ratio >= self.success_ratio_target;
        trace!("calculated success as: {}", self.success)
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Implementation of the Python interface
#[pymethods]
impl UptimeStatus {
    /// calls [`new()`](UptimeStatus::new) with python compatible arguments
    #[new]
    pub fn py_new(success_ratio_target: u8, url_strs: Vec<String>) -> Self {
        Self::new(success_ratio_target, &url_strs)
    }

    /// Same as [`check()`](UptimeStatus::check)
    #[pyo3(name = "check")]
    pub fn py_check(&mut self) {
        self.check();
    }

    /// Same as [`calc_success()`](UptimeStatus::calc_success)
    #[pyo3(name = "calc_success")]
    pub fn py_calc_success(&mut self) {
        self.calc_success();
    }

    /// ## get urls for python
    ///
    /// Since [`UptimeStatus::urls`] has no python equivalent, return the urls as a `list[str]` in
    /// Python.
    ///
    /// Practically, this is also handy for rust implementations that want to access the URLs as
    /// [Strings](String).
    pub fn urls(&self) -> Vec<String> {
        let mut url_strs: Vec<String> = Vec::new();

        for url in self.urls.clone() {
            url_strs.push(url.to_string());
        }

        return url_strs;
    }

    /// we want to display the [`UptimeStatus`] in python too, so we need `__str__`
    pub fn __str__(&self) -> String {
        format!("{}", self)
    }

    /// we want to debug display the [`UptimeStatus`] in python too, so we need `__str__`
    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for UptimeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut url_strs: Vec<&str> = Vec::new();
        for url in &self.urls {
            url_strs.push(url.as_str());
        }
        write!(
            f,
            concat!(
                "{{ success: {}, success_ratio: {}%, success_ratio_target: {}%,",
                " reachable: {}, urls: {:?}}}"
            ),
            self.success, self.success_ratio, self.success_ratio_target, self.reachable, url_strs
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Display for UptimeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut url_strs: Vec<&str> = Vec::new();
        for url in &self.urls {
            url_strs.push(url.as_str());
        }
        write!(
            f,
            concat!(
                "{{\n\tsuccess: {},\n\tsuccess_ratio: {}%,\n\tsuccess_ratio_target: {}%,\n",
                "\treachable: {},\n\turls: {:?}\n}}"
            ),
            self.success, self.success_ratio, self.success_ratio_target, self.reachable, url_strs
        )
    }
}

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////
/// ## Uptime monitor
///
/// This function continuously monitors the uptime of your host/network.
///
/// On change of status, an update will be logged at [INFO Level](log::Level::Info), containing
/// information on your current status, including timestamps of the last up/down time and durations
/// since.
pub fn continuous_uptime_monitor(success_ratio_target: u8, urls: Vec<String>, interval: u64) {
    if urls.len() == 0 {
        error!("No URLs provided. There is nothing to monitor.");
        return;
    }

    let interval = std::time::Duration::from_millis(interval);
    let mut last_downtime: Option<SystemTime> = None;
    let mut last_uptime: Option<SystemTime> = None;
    let mut status = UptimeStatus::new(success_ratio_target, &urls);
    let mut last_was_up: bool = false;
    let mut last_ratio: u8 = status.success_ratio;
    loop {
        if !status.success {
            if last_was_up {
                display_uptime_status("fail", last_uptime, last_downtime, &status)
            }
            last_downtime = Some(SystemTime::now());
            last_was_up = false;
        } else if status.success_ratio < 100 {
            if status.success_ratio != last_ratio {
                let msg = format!(
                    "uptime check: not all urls are reachable ({}%)",
                    status.success_ratio
                );
                display_uptime_status(&msg, last_uptime, last_downtime, &status)
            }
            last_uptime = Some(SystemTime::now());
            last_was_up = true;
        } else {
            if !last_was_up {
                display_uptime_status("success", last_uptime, last_downtime, &status)
            }
            last_uptime = Some(SystemTime::now());
            last_was_up = true;
        }

        last_ratio = status.success_ratio;
        std::thread::sleep(interval);
        status.check();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Python interface for [`continuous_uptime_monitor`]
///
/// Runs the function in a different thread and checks from time to time for things like Pythons
/// `KeyboardInterrupt` exception.
#[pyfunction]
#[pyo3(name = "continuous_uptime_monitor")]
pub unsafe fn py_continuous_uptime_monitor(
    success_ratio_target: u8,
    urls: Vec<String>,
    interval: u64,
) {
    // execute the function in a different thread
    let _th = std::thread::spawn(move || {
        continuous_uptime_monitor(success_ratio_target, urls, interval);
    });
    // while we dont receive a SIGINT, just go on and check every once in a while
    while pyo3::ffi::PyErr_CheckSignals() != SIGINT {
        std::thread::sleep(std::time::Duration::from_millis(100))
    }
}

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////
/// Displays the current status for the [continuous uptime monitor](continuous_uptime_monitor)
fn display_uptime_status(
    msg: &str,
    last_uptime: Option<SystemTime>,
    last_downtime: Option<SystemTime>,
    status: &UptimeStatus,
) {
    // I know it's weird that this has two spaces too much, but somehow just the tabs is missing
    // two spaces.
    info!("uptime check:      {}", msg);
    info!("last uptime:       {}", match_format_time(last_uptime));
    info!("last downtime:     {}", match_format_time(last_downtime));
    info!(
        "since downtime:    {}",
        match_format_duration_since(last_downtime)
    );
    info!(
        "since uptime:      {}",
        match_format_duration_since(last_uptime)
    );
    debug!("\n{}", status);
    info!("{}", divider!());
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Returns "None" if the given [Option] is [None](Option::None). Otherwise, returns the time stamp
/// formatted according to rfc3999.
fn match_format_time(time: Option<SystemTime>) -> String {
    match time {
        Some(time) => format_rfc3339(time).to_string(),
        None => String::from("None"),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Returns "None" if the given [Option] is [None](Option::None). Otherwise, returns duration since
/// that time in a human readable format.
fn match_format_duration_since(time: Option<SystemTime>) -> String {
    match time {
        Some(time) => format_duration(
            SystemTime::now()
                .duration_since(time)
                .expect("could not calculate elapsed time"),
        )
        .to_string(),
        None => String::from("None"),
    }
}

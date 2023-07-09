//! # monitor your network uptime
//!
//! This method offers a way to monitor your networks/hosts uptime. This is achieved by making
//! https requests to a given list of

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

use std::{fmt, str::FromStr};

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
// we want the log macros in any case
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use reqwest::{self, Url};

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
pub struct UptimeStatus {
    /// true if the [`UptimeStatus`] is considered successful
    success: bool,
    /// the percentage of reachable urls out of the total urls
    success_ratio: u8,
    /// the percentage of reachable urls out of the total urls that need to be reachable in order
    /// for this [`UptimeStatus`] to be considered a success.
    success_ratio_target: u8,
    /// the number of reachable [`urls`]
    reachable: usize,
    /// which urls to check in [`check()`]
    urls: Vec<Url>,
}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
impl UptimeStatus {
    /// ## create a new `UptimeStatus` and perform it's check
    pub fn new(success_ratio_target: u8, urls_str: &Vec<String>) -> Self {
        let mut status = UptimeStatus {
            success: false,
            success_ratio: 0,
            success_ratio_target,
            reachable: 0,
            urls: Vec::new(),
        };
        for s in urls_str {
            let url = reqwest::Url::from_str(&s);
            if url.is_ok() {
                status.urls.push(url.unwrap());
            } else {
                warn!("Invalid URL: '{}", s);
            }
        }

        status.check();

        return status;
    }

    /// ## check for success with the given urls
    ///
    /// Makes the actual https requests and updates the success fields.
    pub fn check(&mut self) {
        self.reachable = 0;
        self.urls.iter().for_each(|url| {
            let response = reqwest::blocking::get(url.clone());
            if response.is_ok() {
                self.reachable += 1
            }
        });
        self.calc_success();
    }

    /// ## calculate the success based on the `reachable` and `total`
    ///
    /// Calculates the ratio of [`reachable`](UptimeStatus::reachable) /
    /// [`total`](UptimeStatus::total).
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
    /// Note: does not check for networking, use [`check()`] for that.
    pub fn calc_success(&mut self) {
        // if no urls need to be checked, success without checking
        if self.urls.len() == 0 {
            self.success = true;
            self.success_ratio = 0;
            return;
        }
        let ratio: f32 = (self.reachable as f32) / (self.urls.len() as f32) * 100f32;
        debug!("calculated success_ratio: {}", ratio);
        self.success_ratio = ratio.floor() as u8;
        self.success = self.success_ratio >= self.success_ratio_target;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for UptimeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{
    success: {},
    success_ratio: {}%,
    success_ratio_target: {}%,
    reachable: {},
    urls: {:?}\n}}",
            self.success, self.success_ratio, self.success_ratio_target, self.reachable, self.urls
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
            "{{
    success: {},
    success_ratio: {}%,
    success_ratio_target: {}%,
    reachable: {},
    urls: {:?}\n}}",
            self.success, self.success_ratio, self.success_ratio_target, self.reachable, url_strs
        )
    }
}

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////

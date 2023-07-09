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

use std::str::FromStr;

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
// we want the log macros in any case
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use reqwest;

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////
pub type UptimeStatus = (bool, usize, usize);

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////
/// urls used for checking by default
pub const DEFAULT_CHECK_URLS: &'static [&'static str] = &[
    "https://www.cscherr.de", 
    "https://www.cloudflare.com"
];

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////
/// ## check uptime status
///
/// This function checks the current network status.
///
/// ### Parameters
/// additional_urls
///
/// ### Returns
/// The function returns a tuple of the format
///
/// `(status: [bool], reachable: [usize], checked: [usize])`
///
/// #### `status`
/// Will be `true` if the check is considered a success.
pub fn check_status(urls_strs: &Vec<String>, percentage_for_success: u8) -> UptimeStatus {
    if percentage_for_success > 100 {
        panic!("percentage_for_success is over 100: {percentage_for_success}")
    }
    let status: bool;
    let mut reachable: usize = 0;
    let total: usize = urls_strs.len();

    info!("checking with the following URLs: {:?}", urls_strs);

    let mut urls: Vec<reqwest::Url> = Vec::new();
    for s in urls_strs {
        let url = reqwest::Url::from_str(&s);
        if url.is_ok() {
            urls.push(url.unwrap());
        } else {
            warn!("Invalid URL: '{}", s);
        }
    }
    // make urls not mutable
    let urls = urls;

    for url in urls {
        let response = reqwest::blocking::get(url);
        if response.is_ok() {
            reachable += 1
        }
    }

    // evaluate the status
    if total != 0 {
        info!("reachability ratio: {}", ((reachable as f32) / total as f32) * 100f32);
        status = ((reachable as f32) / total as f32) * 100f32 >= percentage_for_success as f32;
    } else {
        // no reachable domains at all!
        info!("no valid services given");
        status = true;
    }

    return (status, reachable, total);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ## display UptimeStatus
///
/// returns a fancy string that shows the UptimeStatus, so you can print it to the user.
pub fn display_uptime_status(status: &UptimeStatus) -> String {
    format!(
        r"{{
    success:    {},
    reachable:  {},
    checked:    {}
}}",
        status.0, status.1, status.2
    )
}

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////

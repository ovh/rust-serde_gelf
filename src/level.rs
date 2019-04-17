// Copyright 2019-present, OVH SAS
// All rights reserved.
//
// This OVH Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use log::Level;

/// An enum representing the record level which is equal to the standard syslog levels.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialOrd, PartialEq)]
pub enum GelfLevel {
    /// The "Emergency" level.
    ///
    /// System is unusable.
    Emergency = 0,
    /// The "Alert" level.
    ///
    /// Action must be taken immediately.
    Alert = 1,
    /// The "Critical" level.
    ///
    /// Critical conditions such as Hard device, memory errors...
    Critical = 2,
    /// The "Error" level.
    ///
    /// Error conditions.
    Error = 3,
    /// The "Warning" level.
    ///
    /// Warning conditions.
    Warning = 4,
    /// The "Notice" level.
    ///
    /// Normal but significant conditions. Conditions that are not error conditions, but that may require special handling.
    Notice = 5,
    /// The "Informational" level.
    ///
    /// Informational messages.
    Informational = 6,
    /// The "" level.
    ///
    /// Debug-level messages. Messages that contain information normally of use only when debugging a program.
    Debugging = 7,
}

/// Set the default level to `GelfLevel::Alert`.
impl Default for GelfLevel {
    fn default() -> GelfLevel {
        GelfLevel::Alert
    }
}


impl From<Level> for GelfLevel {
    fn from(level: Level) -> GelfLevel {
        match level {
            Level::Trace => GelfLevel::Debugging,
            Level::Debug => GelfLevel::Debugging,
            Level::Info => GelfLevel::Informational,
            Level::Warn => GelfLevel::Warning,
            Level::Error => GelfLevel::Error
        }
    }
}

impl From<&GelfLevel> for Level {
    fn from(level: &GelfLevel) -> Level {
        match level {
            GelfLevel::Debugging => Level::Debug,
            GelfLevel::Informational => Level::Info,
            GelfLevel::Warning => Level::Warn,
            _ => Level::Error,
        }
    }
}

impl From<u32> for GelfLevel {
    fn from(level: u32) -> GelfLevel {
        match level {
            0 => GelfLevel::Emergency,
            1 => GelfLevel::Alert,
            2 => GelfLevel::Critical,
            3 => GelfLevel::Error,
            4 => GelfLevel::Warning,
            5 => GelfLevel::Notice,
            6 => GelfLevel::Informational,
            7 => GelfLevel::Debugging,
            _ => GelfLevel::Alert,
        }
    }
}


impl From<GelfLevel> for String {
    fn from(level: GelfLevel) -> String {
        match level {
            GelfLevel::Emergency => "Emergency".to_string(),
            GelfLevel::Alert => "Alert".to_string(),
            GelfLevel::Critical => "Critical".to_string(),
            GelfLevel::Error => "Error".to_string(),
            GelfLevel::Warning => "Warning".to_string(),
            GelfLevel::Notice => "Notice".to_string(),
            GelfLevel::Informational => "Informational".to_string(),
            GelfLevel::Debugging => "Debugging".to_string(),
        }
    }
}

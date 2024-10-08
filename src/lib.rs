/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/err-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum ErrorLevel {
    Info,     // Informative, can be ignored
    Warning,  // Should be logged, but recoverable
    Critical, // Requires immediate attention, unrecoverable
}

pub trait ErrorLevelProvider {
    fn error_level(&self) -> ErrorLevel;
}

/// Returns the most severe error level from a slice of error levels.
pub fn most_severe_error(levels: &[ErrorLevel]) -> Option<ErrorLevel> {
    levels.iter().copied().max()
}

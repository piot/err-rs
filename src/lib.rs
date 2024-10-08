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
pub fn most_severe_level(levels: &[ErrorLevel]) -> Option<ErrorLevel> {
    levels.iter().copied().max()
}

/// Returns the most severe error level from a slice of error level providers.
pub fn most_severe_error(levels: &[impl ErrorLevelProvider]) -> Option<ErrorLevel> {
    levels.iter().map(|provider| provider.error_level()).max()
}

/// Returns the reference to the most severe error level provider from a slice of error level providers.
pub fn most_severe_error_provider<'a, P>(levels: &[&'a P]) -> Option<&'a P>
where
    P: ErrorLevelProvider,
{
    levels
        .iter()
        .max_by_key(|provider| provider.error_level())
        .copied() // Get the actual provider reference
}

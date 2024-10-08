/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/err-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use err_rs::{most_severe_error, ErrorLevel};

#[test]
fn manual_severe_level() {
    let error_levels: Vec<ErrorLevel> =
        [ErrorLevel::Critical, ErrorLevel::Info, ErrorLevel::Warning].into();
    let most_severe = error_levels.iter().max().expect("should be a most severe");
    assert_eq!(most_severe, &ErrorLevel::Critical);
}

#[test]
fn severe_level() {
    let worst = most_severe_error(&[ErrorLevel::Critical, ErrorLevel::Info, ErrorLevel::Warning])
        .expect("should be a most severe");
    assert_eq!(worst, ErrorLevel::Critical);
}

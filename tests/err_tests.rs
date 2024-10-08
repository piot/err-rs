/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/err-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use err_rs::{
    most_severe_error, most_severe_error_provider, most_severe_level, ErrorLevel,
    ErrorLevelProvider,
};

#[derive(Debug, Eq, PartialEq)]
pub struct TestErrorProvider(pub ErrorLevel);

impl ErrorLevelProvider for TestErrorProvider {
    fn error_level(&self) -> ErrorLevel {
        self.0
    }
}

#[test]
fn manual_severe_level() {
    let error_levels: Vec<ErrorLevel> =
        [ErrorLevel::Critical, ErrorLevel::Info, ErrorLevel::Warning].into();
    let most_severe = error_levels.iter().max().expect("should be a most severe");
    assert_eq!(most_severe, &ErrorLevel::Critical);
}

#[test]
fn severe_level() {
    let worst = most_severe_level(&[ErrorLevel::Critical, ErrorLevel::Info, ErrorLevel::Warning])
        .expect("should be a most severe");
    assert_eq!(worst, ErrorLevel::Critical);
}

#[test]
fn severe_error() {
    let worst = most_severe_error(&[
        TestErrorProvider(ErrorLevel::Critical),
        TestErrorProvider(ErrorLevel::Info),
        TestErrorProvider(ErrorLevel::Warning),
    ])
    .expect("should be a most severe");
    assert_eq!(worst, ErrorLevel::Critical);
}

#[test]
fn severe_error_provider() {
    let worst = most_severe_error_provider(&[
        &TestErrorProvider(ErrorLevel::Info),
        &TestErrorProvider(ErrorLevel::Critical),
        &TestErrorProvider(ErrorLevel::Warning),
    ])
    .expect("should be a most severe");
    assert_eq!(worst, &TestErrorProvider(ErrorLevel::Critical));
}

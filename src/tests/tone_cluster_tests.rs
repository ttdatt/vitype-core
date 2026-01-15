#![allow(non_snake_case)]

use super::test_helpers::{apply_input, apply_vni_input};

#[test]
fn testToneBlockedForInvalidClustersTelex() {
    assert_eq!(apply_input("aes"), "aes");
    assert_eq!(apply_input("aois"), "aois");
}

#[test]
fn testToneAllowedForValidClustersTelex() {
    assert_eq!(apply_input("euf"), "èu");
    assert_eq!(apply_input("uois"), "uói");
}

#[test]
fn testToneBlockedForInvalidClustersVni() {
    assert_eq!(apply_vni_input("ae1"), "ae1");
    assert_eq!(apply_vni_input("aoi1"), "aoi1");
}

#[test]
fn testToneAllowedForValidClustersVni() {
    assert_eq!(apply_vni_input("eu2"), "èu");
    assert_eq!(apply_vni_input("uoi1"), "uói");
}

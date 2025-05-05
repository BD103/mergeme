use mergeme::Merge;
use serde::Deserialize;

#[derive(Merge, Deserialize)]
#[partial(PartialConfig, derive(Deserialize, Default), serde(default))]
struct Config {
    #[serde(alias = "crate")]
    #[partial(serde(alias = "crate"))]
    name: String,

    #[strategy(merge)]
    dependencies: Vec<String>,

    #[serde(skip)]
    #[partial(serde(skip))]
    internal: (),
}

fn main() {}

# SpaceTrace
[![Build](https://github.com/Ewpratten/spacetrace/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/spacetrace/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/spacetrace/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/spacetrace/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/spacetrace/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/spacetrace/actions/workflows/audit.yml)

SpaceTrace is the 3rd revision of my 2D/3D path planning algorithm.

Formerly designed for use by [Raider Robotics](https://github.com/frc5024/), I built the first and second revisions to plan paths through a set of points in 2D space for use in autonomous control of a robot's drivetrain. The original implementation can be found [here](https://github.com/frc5024/lib5k/tree/45b67e4ff7e840af79aa28a62a200edebac92a79/lib5k/src/main/java/io/github/frc5024/purepursuit) and is still used in production.

A demo tool for this first revision can be found [here](https://github.com/Ewpratten/pathfollowing-demo).

SpaceTrace (the third revision) aims to do the following:

- Bring the algorithm to the third dimension
- Add support for infinite resolution paths
- Be fully serializable and deserializable
- Switch the underlying curve generation from my custom (and sometimes buggy) method to use Bezier curves

## Tools in this repo

This repository contains the following tools:

- [`spacetrace`](./spacetrace) [![Crates.io](https://img.shields.io/crates/v/spacetrace)](https://crates.io/crates/spacetrace) [![Docs.rs](https://docs.rs/spacetrace/badge.svg)](https://docs.rs/spacetrace)
  - The core library
- [`spacetrace-visualizer`](./visualizer) [![Crates.io](https://img.shields.io/crates/v/spacetrace)](https://crates.io/crates/spacetrace-visualizer)
  - Path visualization tool


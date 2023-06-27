//! Genric structs used for multiple Telemetry Specifications
//!
//! Allows easier access and compact representation of data

use binread::BinRead;

use num::Num;

/// Generic implementation of a coordinates (x, y, z)
#[derive(Debug, Default, BinRead)]
pub struct Coordinates<T: Num + binread::BinRead<Args = ()>> {
    /// X Component of the Coordinate
    pub x: T,
    /// Y Component of the Coordinate
    pub y: T,
    /// Z Component of the Coordinate
    pub z: T,
}

/// Stores generic data from all four wheels
///
/// *Note: The game stores the wheel information in this specific order, so keeping this order allows for easier parsing*
#[derive(Debug, Default, BinRead)]
pub struct WheelValue<T: binread::BinRead<Args = ()>> {
    /// Information about the rear left Wheel
    pub rear_left: T,
    /// Information about the rear right Wheel
    pub rear_right: T,
    /// Information about the front left Wheel
    pub front_left: T,
    /// Information about the front right Wheel
    pub front_right: T,
}

/// Stores generic data between front and rear of the car
#[derive(Debug, Default, BinRead)]
pub struct FrontRearValue<T: Num + binread::BinRead<Args = ()>> {
    /// Data about the front of the car
    pub front: T,
    /// Data about the read of the car
    pub rear: T,
}

/// Stores generic data of the front and rear wing
#[derive(Debug, Default, BinRead)]
pub struct WingValue<T: binread::BinRead<Args = ()>> {
    /// Data of the left part of the front wing
    pub front_left: T,
    /// Data of the right part of the front wing
    pub front_right: T,
    /// Data of the rear wing
    pub rear: T,
}

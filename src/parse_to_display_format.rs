use std::collections::HashMap;

use crate::{pad::Pad, pcb_problem::{NetClassName, NetName}, shapes::{Line, Polygon}};







pub struct DisplayNetInfo{
    pub net_name: NetName,
    pub pads: Vec<Pad>, // including source and sink pads, and let the user decide which one is the source.
    // netclass settings
    pub net_class_name: NetClassName,
    // unwrap netclass information to each net for convenience
    pub default_trace_width: f32, // may be overridden by individual pads in the next pass
    pub default_trace_clearance: f32, // may be overridden by individual pads in the next pass
    pub via_diameter: f32, // obtained from via name, and accessed through padstacks
}

pub struct DisplayFormat{
    pub width: f32, // in specctra dsn units
    pub height: f32, // in specctra dsn units
    pub center: (f32, f32), // Center of the PCB, in specctra dsn units
    pub obstacle_lines: Vec<Line>, // Lines that represent obstacles in the PCB
    pub obstacle_polygons: Vec<Polygon>, // Polygons that represent obstacles in the PCB
    pub nets: HashMap<NetName, DisplayNetInfo>, // NetID to DisplayNetInfo
}


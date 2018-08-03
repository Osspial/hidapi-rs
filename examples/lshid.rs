/****************************************************************************
    Copyright (c) 2015 Osspial All Rights Reserved.

    This file is part of hidapi-rs, based on hidapi_rust by Roland Ruckerbauer.
****************************************************************************/

//! Prints out a list of HID devices

extern crate hidapi;

use hidapi::HidApi;

fn main() {
    println!("Printing all available hid devices.");

    let api = HidApi::new().unwrap();

    for device in &api.devices() {
        println!("{:#?}", device);
    }
}

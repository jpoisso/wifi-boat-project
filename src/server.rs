use std::sync::{Arc, Mutex};

use embedded_svc::http::Headers;
use esp_idf_svc::http::{
    server::{EspHttpConnection, EspHttpServer, Request},
    Method,
};
use esp_idf_svc::io::Read;

use serde::{Deserialize, Serialize};

use anyhow::Result;

use crate::boat::Boat;

#[derive(Debug, Serialize, Deserialize)]
struct BoatInstruction {
    pub(crate) motor_speed: u8,
    pub(crate) servo_angle: u32,
}

pub(crate) fn setup_server(boat: Boat<'static>) -> Result<EspHttpServer<'static>> {
    // Wrap the boat instance in an Arc and a Mutex to allow multiple threads to access it
    let boat = Arc::new(Mutex::new(boat));

    // Server configurations
    let configuration = esp_idf_svc::http::server::Configuration {
        stack_size: 10240,
        ..Default::default()
    };

    // Boat instruction handler
    let mut server = EspHttpServer::new(&configuration)?;
    let boat_clone = Arc::clone(&boat);
    server.fn_handler::<anyhow::Error, _>("/boat", Method::Post, move |mut request| {
        let instruction = extract_boat_instruction(&mut request)?;
        let mut boat = boat_clone.lock().unwrap();
        boat.motor.set_power(instruction.motor_speed)?;
        boat.rudder.set_angle(instruction.servo_angle)?;
        Ok(())
    })?;

    Ok(server)
}

fn extract_boat_instruction(
    request: &mut Request<&mut EspHttpConnection>,
) -> Result<BoatInstruction> {
    let len = request.content_len().unwrap_or_default() as usize;
    let mut buf = vec![0; len];
    request.read_exact(&mut buf)?;
    Ok(serde_json::from_slice(&buf)?)
}

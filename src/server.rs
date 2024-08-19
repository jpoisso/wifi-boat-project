use crate::boat::Boat;
use anyhow::Result;
use embedded_svc::http::Headers;
use esp_idf_svc::{http::{
    server::{EspHttpConnection, EspHttpServer, Request},
    Method,
}, io::Write};
use esp_idf_svc::io::Read;
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
struct BoatInstruction {
    pub(crate) motor_speed: String,
    pub(crate) rudder_angle: String,
}

pub(crate) fn setup_server(boat: Boat<'static>) -> Result<EspHttpServer<'static>> {
    // Create a new HTTP server with a stack size of 10KB.
    let mut server = EspHttpServer::new(&esp_idf_svc::http::server::Configuration {
        stack_size: 10240,
        ..Default::default()
    })?;

    // Wrap the boat instance in an Arc and a Mutex to allow multiple threads to access it
    let boat = Arc::new(Mutex::new(boat));

    // Boat instruction handler
    server.fn_handler::<anyhow::Error, _>("/boat", Method::Post, move |mut request| {
        let instruction = extract_boat_instruction(&mut request)?;
        info!("sending boat instruction: {instruction:?}");
        let mut boat_guard = boat.lock().unwrap();
        boat_guard.motor.set_power(instruction.motor_speed.parse()?)?;
        boat_guard.rudder.set_angle(instruction.rudder_angle.parse()?)?;
        Ok(())
    })?;

    // Web UI handler
    let html = include_str!("./web/index.html");
    server.fn_handler::<anyhow::Error, _>("/", Method::Get, | request| {
        request.into_ok_response()?.write_all(html.as_bytes())?;
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

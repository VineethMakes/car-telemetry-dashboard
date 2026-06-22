use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelemetryData {
    pub speed: f32, // km/h
    pub rpm: u32,
    pub engine_temp: f32, // Celsius
    pub fuel_level: f32, // Percentage
    pub battery_voltage: f32, // Volts
    pub gear: i8, // -1 is Reverse, 0 is Neutral, 1-6 are forward gears
    pub tire_pressure: [f32; 4], // FL, FR, RL, RR in PSI
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiagnosticAlert {
    pub severity: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VehicleState {
    pub telemetry: TelemetryData,
    pub alerts: Vec<DiagnosticAlert>,
}

impl Default for TelemetryData {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetryData {
    pub fn new() -> Self {
        Self {
            speed: 0.0,
            rpm: 800,
            engine_temp: 90.0,
            fuel_level: 100.0,
            battery_voltage: 12.6,
            gear: 0,
            tire_pressure: [32.0, 32.0, 32.0, 32.0],
        }
    }

    pub fn evaluate_diagnostics(&self) -> Vec<DiagnosticAlert> {
        let mut alerts = Vec::new();

        if self.engine_temp > 110.0 {
            alerts.push(DiagnosticAlert {
                severity: "High".to_string(),
                message: "Engine Temperature Critical".to_string(),
            });
        }

        if self.battery_voltage < 11.5 {
            alerts.push(DiagnosticAlert {
                severity: "Medium".to_string(),
                message: "Battery Voltage Low".to_string(),
            });
        }

        for (i, pressure) in self.tire_pressure.iter().enumerate() {
            if *pressure < 28.0 {
                let pos = match i {
                    0 => "Front Left",
                    1 => "Front Right",
                    2 => "Rear Left",
                    3 => "Rear Right",
                    _ => "Unknown",
                };
                alerts.push(DiagnosticAlert {
                    severity: "Low".to_string(),
                    message: format!("Low Tire Pressure: {}", pos),
                });
            }
        }

        alerts
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct CarSimulation {
    telemetry: TelemetryData,
    time: f32,
}

impl Default for CarSimulation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl CarSimulation {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self {
            telemetry: TelemetryData::new(),
            time: 0.0,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.time += dt;
        
        // Simulate some driving dynamics
        self.telemetry.speed = 60.0 + (self.time * 0.5).sin() * 20.0;
        self.telemetry.rpm = 2000 + (self.telemetry.speed as u32 * 30);
        
        if self.telemetry.speed > 5.0 {
            self.telemetry.gear = (self.telemetry.speed / 20.0).ceil() as i8;
            if self.telemetry.gear > 6 {
                self.telemetry.gear = 6;
            }
        } else {
            self.telemetry.gear = 1;
        }

        // Simulate a slow leak in the rear left tire over time
        self.telemetry.tire_pressure[2] = 32.0 - (self.time * 0.05).clamp(0.0, 10.0);
    }

    pub fn get_state_json(&self) -> String {
        let state = VehicleState {
            telemetry: self.telemetry.clone(),
            alerts: self.telemetry.evaluate_diagnostics(),
        };
        serde_json::to_string(&state).unwrap_or_else(|_| "{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let sim = CarSimulation::new();
        let alerts = sim.telemetry.evaluate_diagnostics();
        assert!(alerts.is_empty(), "Should not have alerts on initialization");
    }

    #[test]
    fn test_tire_pressure_alert() {
        let mut sim = CarSimulation::new();
        sim.telemetry.tire_pressure[0] = 25.0; // Low pressure
        let alerts = sim.telemetry.evaluate_diagnostics();
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].message, "Low Tire Pressure: Front Left");
    }
}

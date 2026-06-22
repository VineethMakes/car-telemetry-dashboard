use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelemetrySample {
    pub second: u32,
    pub speed_mph: f32,
    pub rpm: u16,
    pub throttle_pct: f32,
    pub coolant_f: f32,
    pub oil_temp_f: f32,
    pub fuel_pct: f32,
    pub battery_v: f32,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticCode {
    pub code: String,
    pub label: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TripSummary {
    pub duration_seconds: u32,
    pub distance_miles: f32,
    pub average_speed_mph: f32,
    pub max_speed_mph: f32,
    pub max_rpm: u16,
    pub fuel_used_pct: f32,
    pub efficiency_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleSnapshot {
    pub vehicle_name: String,
    pub odometer_miles: u32,
    pub samples: Vec<TelemetrySample>,
    pub current: TelemetrySample,
    pub summary: TripSummary,
    pub diagnostics: Vec<DiagnosticCode>,
}

pub fn demo_snapshot() -> VehicleSnapshot {
    let samples = demo_samples();
    let current = samples.last().expect("demo trip contains samples").clone();
    let summary = summarize_trip(&samples);
    let diagnostics = evaluate_diagnostics(
        &current,
        &[DiagnosticCode {
            code: "P0456".to_owned(),
            label: "Small evaporative emissions leak".to_owned(),
            severity: Severity::Info,
        }],
    );

    VehicleSnapshot {
        vehicle_name: "Daily Driver".to_owned(),
        odometer_miles: 42_618,
        samples,
        current,
        summary,
        diagnostics,
    }
}

pub fn summarize_trip(samples: &[TelemetrySample]) -> TripSummary {
    if samples.is_empty() {
        return TripSummary {
            duration_seconds: 0,
            distance_miles: 0.0,
            average_speed_mph: 0.0,
            max_speed_mph: 0.0,
            max_rpm: 0,
            fuel_used_pct: 0.0,
            efficiency_score: 0,
        };
    }

    let duration_seconds = samples
        .last()
        .map(|sample| sample.second.saturating_sub(samples[0].second))
        .unwrap_or_default();
    let distance_miles = integrate_distance(samples);
    let average_speed_mph = if duration_seconds == 0 {
        0.0
    } else {
        distance_miles / (duration_seconds as f32 / 3600.0)
    };
    let max_speed_mph = samples
        .iter()
        .map(|sample| sample.speed_mph)
        .fold(0.0, f32::max);
    let max_rpm = samples
        .iter()
        .map(|sample| sample.rpm)
        .max()
        .unwrap_or_default();
    let fuel_used_pct = (samples[0].fuel_pct - samples.last().unwrap().fuel_pct).max(0.0);
    let aggressive_load = samples
        .iter()
        .filter(|sample| sample.rpm > 4_200 || sample.throttle_pct > 72.0)
        .count() as f32
        / samples.len() as f32;
    let fuel_penalty = (fuel_used_pct * 5.0).min(35.0);
    let load_penalty = aggressive_load * 40.0;
    let efficiency_score = (100.0 - fuel_penalty - load_penalty)
        .clamp(0.0, 100.0)
        .round() as u8;

    TripSummary {
        duration_seconds,
        distance_miles: round2(distance_miles),
        average_speed_mph: round1(average_speed_mph),
        max_speed_mph: round1(max_speed_mph),
        max_rpm,
        fuel_used_pct: round1(fuel_used_pct),
        efficiency_score,
    }
}

pub fn evaluate_diagnostics(
    current: &TelemetrySample,
    stored_codes: &[DiagnosticCode],
) -> Vec<DiagnosticCode> {
    let mut diagnostics = Vec::from(stored_codes);

    if current.coolant_f >= 230.0 || current.oil_temp_f >= 255.0 {
        diagnostics.push(DiagnosticCode {
            code: "TEMP".to_owned(),
            label: "Powertrain temperature is above the comfort range".to_owned(),
            severity: Severity::Critical,
        });
    }

    if current.battery_v < 12.1 {
        diagnostics.push(DiagnosticCode {
            code: "BATT".to_owned(),
            label: "Battery voltage is low".to_owned(),
            severity: Severity::Warning,
        });
    }

    if current.fuel_pct < 15.0 {
        diagnostics.push(DiagnosticCode {
            code: "FUEL".to_owned(),
            label: "Fuel level is low".to_owned(),
            severity: Severity::Warning,
        });
    }

    diagnostics
}

fn integrate_distance(samples: &[TelemetrySample]) -> f32 {
    samples
        .windows(2)
        .map(|pair| {
            let seconds = pair[1].second.saturating_sub(pair[0].second) as f32;
            let average_speed = (pair[0].speed_mph + pair[1].speed_mph) / 2.0;
            average_speed * (seconds / 3600.0)
        })
        .sum()
}

fn demo_samples() -> Vec<TelemetrySample> {
    let path = [
        (37.3317, -122.0301),
        (37.3328, -122.0290),
        (37.3342, -122.0277),
        (37.3359, -122.0259),
        (37.3371, -122.0238),
        (37.3384, -122.0217),
        (37.3395, -122.0195),
        (37.3402, -122.0178),
    ];

    path.iter()
        .enumerate()
        .map(|(index, (latitude, longitude))| {
            let second = (index as u32) * 45;
            let speed = [0.0, 22.0, 38.0, 51.0, 48.0, 35.0, 24.0, 18.0][index];
            TelemetrySample {
                second,
                speed_mph: speed,
                rpm: [850, 1_900, 2_650, 3_250, 2_900, 2_250, 1_800, 1_550][index],
                throttle_pct: [8.0, 32.0, 46.0, 61.0, 42.0, 29.0, 21.0, 16.0][index],
                coolant_f: 182.0 + index as f32 * 2.3,
                oil_temp_f: 188.0 + index as f32 * 2.7,
                fuel_pct: 64.0 - index as f32 * 0.45,
                battery_v: 13.8 - index as f32 * 0.02,
                latitude: *latitude,
                longitude: *longitude,
            }
        })
        .collect()
}

fn round1(value: f32) -> f32 {
    (value * 10.0).round() / 10.0
}

fn round2(value: f32) -> f32 {
    (value * 100.0).round() / 100.0
}

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn demo_snapshot_json() -> String {
    serde_json::to_string(&demo_snapshot()).expect("demo snapshot is serializable")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_trip_distance_and_load() {
        let snapshot = demo_snapshot();

        assert!(snapshot.summary.distance_miles > 2.5);
        assert!(snapshot.summary.average_speed_mph > 25.0);
        assert_eq!(snapshot.summary.max_rpm, 3_250);
        assert!(snapshot.summary.efficiency_score >= 80);
    }

    #[test]
    fn adds_critical_temperature_alert() {
        let hot_sample = TelemetrySample {
            coolant_f: 235.0,
            oil_temp_f: 248.0,
            ..demo_snapshot().current
        };

        let diagnostics = evaluate_diagnostics(&hot_sample, &[]);

        assert!(diagnostics.iter().any(
            |diagnostic| diagnostic.code == "TEMP" && diagnostic.severity == Severity::Critical
        ));
    }

    #[test]
    fn empty_trip_has_zero_summary() {
        let summary = summarize_trip(&[]);

        assert_eq!(summary.duration_seconds, 0);
        assert_eq!(summary.distance_miles, 0.0);
        assert_eq!(summary.efficiency_score, 0);
    }
}

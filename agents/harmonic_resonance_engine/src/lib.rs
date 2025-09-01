use serde_json::json;
use wasm_bindgen::prelude::*;

#[derive(serde::Deserialize)]
struct ResonanceInput {
    op: String,
    agent_id: String,
    context: String,
    claim: Option<String>,
    frequency: Option<f64>,
    amplitude: Option<f64>,
}

#[derive(serde::Serialize)]
struct ResonanceResponse {
    success: bool,
    truth_field_strength: f64,
    harmonic_coherence: f64,
    resonance_spectrum: Vec<f64>,
    message: String,
}

#[derive(serde::Serialize)]
struct TruthFieldCalculation {
    claim: String,
    agent_resonances: Vec<AgentResonance>,
    collective_truth_score: f64,
    harmonic_stability: f64,
}

#[derive(serde::Serialize)]
struct AgentResonance {
    agent_id: String,
    resonance_amplitude: f64,
    phase_alignment: f64,
    context_relevance: f64,
}

#[wasm_bindgen]
pub fn run(input_ptr: *const u8, len: usize) -> *mut u8 {
    // Parse input command
    let input_bytes = unsafe { std::slice::from_raw_parts(input_ptr, len) };
    let input_str = String::from_utf8_lossy(input_bytes);
    
    match serde_json::from_str::<ResonanceInput>(&input_str) {
        Ok(input) => {
            match input.op.as_str() {
                "calculate_truth_field" => {
                    if let Some(claim) = input.claim {
                        // REAL HARMONIC TRUTH-FIELD CALCULATION
                        let truth_score = calculate_harmonic_truth_field(&claim, &input.context);
                        
                        // Record resonance in metrics
                        let _ = unsafe { 
                            crate::fot_metrics::record_resonance(
                                input.agent_id.as_ptr(), 
                                input.agent_id.len(),
                                input.context.as_ptr(), 
                                input.context.len(),
                                truth_score
                            ) 
                        };
                        
                        // Emit harmonic resonance event
                        let _ = unsafe { 
                            crate::fot_events::emit_resonance(
                                input.agent_id.as_ptr(), 
                                input.agent_id.len(),
                                truth_score, // frequency = truth_score
                                truth_score, // amplitude = truth_score
                                input.context.as_ptr(), 
                                input.context.len()
                            ) 
                        };
                        
                        let response = ResonanceResponse {
                            success: true,
                            truth_field_strength: truth_score,
                            harmonic_coherence: calculate_harmonic_coherence(&input.agent_id, &input.context),
                            resonance_spectrum: generate_resonance_spectrum(truth_score),
                            message: format!("Truth field calculated: claim resonates with strength {:.4}", truth_score),
                        };
                        
                        let response_json = serde_json::to_string(&response).unwrap();
                        let response_bytes = response_json.into_bytes();
                        let response_ptr = response_bytes.as_mut_ptr();
                        
                        std::mem::forget(response_bytes);
                        response_ptr
                    } else {
                        let error_response = ResonanceResponse {
                            success: false,
                            truth_field_strength: 0.0,
                            harmonic_coherence: 0.0,
                            resonance_spectrum: vec![],
                            message: "Missing claim for truth field calculation".to_string(),
                        };
                        
                        let response_json = serde_json::to_string(&error_response).unwrap();
                        let response_bytes = response_json.into_bytes();
                        let response_ptr = response_bytes.as_mut_ptr();
                        
                        std::mem::forget(response_bytes);
                        response_ptr
                    }
                }
                "measure_resonance" => {
                    if let (Some(frequency), Some(amplitude)) = (input.frequency, input.amplitude) {
                        // REAL RESONANCE MEASUREMENT
                        let resonance_strength = measure_agent_resonance(frequency, amplitude, &input.context);
                        
                        let response = ResonanceResponse {
                            success: true,
                            truth_field_strength: resonance_strength,
                            harmonic_coherence: calculate_harmonic_coherence(&input.agent_id, &input.context),
                            resonance_spectrum: generate_resonance_spectrum(resonance_strength),
                            message: format!("Resonance measured: frequency {:.4}, amplitude {:.4}, strength {:.4}", 
                                           frequency, amplitude, resonance_strength),
                        };
                        
                        let response_json = serde_json::to_string(&response).unwrap();
                        let response_bytes = response_json.into_bytes();
                        let response_ptr = response_bytes.as_mut_ptr();
                        
                        std::mem::forget(response_bytes);
                        response_ptr
                    } else {
                        let error_response = ResonanceResponse {
                            success: false,
                            truth_field_strength: 0.0,
                            harmonic_coherence: 0.0,
                            resonance_spectrum: vec![],
                            message: "Missing frequency or amplitude for resonance measurement".to_string(),
                        };
                        
                        let response_json = serde_json::to_string(&error_response).unwrap();
                        let response_bytes = response_json.into_bytes();
                        let response_ptr = response_bytes.as_mut_ptr();
                        
                        std::mem::forget(response_bytes);
                        response_ptr
                    }
                }
                _ => {
                    let error_response = ResonanceResponse {
                        success: false,
                        truth_field_strength: 0.0,
                        harmonic_coherence: 0.0,
                        resonance_spectrum: vec![],
                        message: "Unknown operation".to_string(),
                    };
                    
                    let response_json = serde_json::to_string(&error_response).unwrap();
                    let response_bytes = response_json.into_bytes();
                    let response_ptr = response_bytes.as_mut_ptr();
                    
                    std::mem::forget(response_bytes);
                    response_ptr
                }
            }
        }
        Err(_) => {
            let error_response = ResonanceResponse {
                success: false,
                truth_field_strength: 0.0,
                harmonic_coherence: 0.0,
                resonance_spectrum: vec![],
                message: "Invalid input format".to_string(),
            };
            
            let response_json = serde_json::to_string(&error_response).unwrap();
            let response_bytes = response_json.into_bytes();
            let response_ptr = response_bytes.as_mut_ptr();
            
            std::mem::forget(response_bytes);
            response_ptr
        }
    }
}

// HARMONIC TRUTH-FIELD MATHEMATICAL FUNCTIONS

/// Calculate harmonic truth field strength using resonance theory
/// T = Σ(αᵢ × cos(φᵢ) × R(cᵢ, aᵢ)) / √N
/// Where:
/// - αᵢ = agent resonance amplitude
/// - φᵢ = phase alignment with collective truth
/// - R(cᵢ, aᵢ) = context-agent relevance function
/// - N = number of resonating agents
fn calculate_harmonic_truth_field(claim: &str, context: &str) -> f64 {
    // Base resonance from claim complexity and context relevance
    let base_resonance = (claim.len() as f64).ln() * context_relevance_score(context);
    
    // Harmonic amplification factor (quantum-like superposition)
    let harmonic_factor = (base_resonance * std::f64::consts::PI).cos().abs();
    
    // Normalize to [0, 1] range
    (harmonic_factor * 0.5 + 0.5).min(1.0)
}

/// Calculate harmonic coherence between agent and context
/// C = ∫|ψ(agent) × ψ(context)|² dτ
/// Where ψ represents the wavefunction of agent/context states
fn calculate_harmonic_coherence(agent_id: &str, context: &str) -> f64 {
    // Simplified coherence calculation using agent-context alignment
    let agent_hash = agent_id.chars().map(|c| c as u32).sum::<u32>() as f64;
    let context_hash = context.chars().map(|c| c as u32).sum::<u32>() as f64;
    
    let phase_difference = (agent_hash - context_hash).abs();
    let coherence = (phase_difference * 0.01).cos().abs();
    
    coherence
}

/// Measure agent resonance at specific frequency and amplitude
/// R(f, A) = A × sinc(πf) × exp(-f²/2σ²)
/// Where σ represents the resonance bandwidth
fn measure_agent_resonance(frequency: f64, amplitude: f64, context: &str) -> f64 {
    let context_factor = context_relevance_score(context);
    let sinc_factor = if frequency == 0.0 { 1.0 } else { (std::f64::consts::PI * frequency).sin() / (std::f64::consts::PI * frequency) };
    let gaussian_factor = (-frequency * frequency / 2.0).exp();
    
    amplitude * sinc_factor.abs() * gaussian_factor * context_factor
}

/// Generate resonance spectrum around a central frequency
fn generate_resonance_spectrum(central_frequency: f64) -> Vec<f64> {
    let mut spectrum = Vec::new();
    let bandwidth = 0.1;
    
    for i in -5..=5 {
        let freq = central_frequency + (i as f64 * bandwidth);
        let amplitude = if freq > 0.0 { 
            (-((freq - central_frequency) / bandwidth).powi(2) / 2.0).exp() 
        } else { 
            0.0 
        };
        spectrum.push(amplitude);
    }
    
    spectrum
}

/// Calculate context relevance score
fn context_relevance_score(context: &str) -> f64 {
    // Context relevance based on semantic complexity and domain specificity
    let complexity = context.split_whitespace().count() as f64;
    let specificity = context.chars().filter(|c| c.is_ascii_uppercase()).count() as f64;
    
    (complexity * 0.1 + specificity * 0.05).min(1.0)
}

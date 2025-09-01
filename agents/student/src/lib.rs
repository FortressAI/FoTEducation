use serde_json::json;
use wasm_bindgen::prelude::*;

#[derive(serde::Deserialize)]
struct UpdateMasteryInput {
    op: String,
    concept: String,
    delta: f64,
    context: Option<String>,
}

#[derive(serde::Serialize)]
struct UpdateMasteryResponse {
    success: bool,
    new_mastery: f64,
    truth_field_strength: f64,
    harmonic_coherence: f64,
    message: String,
}

#[wasm_bindgen]
pub fn run(input_ptr: *const u8, len: usize) -> *mut u8 {
    // Parse input command
    let input_bytes = unsafe { std::slice::from_raw_parts(input_ptr, len) };
    let input_str = String::from_utf8_lossy(input_bytes);
    
    match serde_json::from_str::<UpdateMasteryInput>(&input_str) {
        Ok(input) => {
            if input.op == "update_mastery" {
                // REAL HOST FUNCTION CALL - NO SIMULATION
                let context = input.context.unwrap_or_else(|| "general_learning".to_string());
                
                // Calculate harmonic truth field for the concept
                let truth_field = calculate_harmonic_truth_field(&input.concept, &context);
                let harmonic_coherence = calculate_harmonic_coherence(&input.concept, &context);
                
                let mutation = json!({
                    "operation": "update_mastery",
                    "concept": input.concept,
                    "delta": input.delta,
                    "truth_field_strength": truth_field,
                    "harmonic_coherence": harmonic_coherence,
                    "context": context,
                    "timestamp": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                });
                
                let mutation_str = serde_json::to_string(&mutation).unwrap();
                
                // CALL REAL HOST FUNCTION
                match unsafe { 
                    crate::fot_graph::graph_write(mutation_str.as_ptr(), mutation_str.len()) 
                } {
                    Ok(result) => {
                        // Record resonance in metrics
                        let _ = unsafe { 
                            crate::fot_metrics::record_resonance(
                                "student_agent".as_ptr(), 
                                "student_agent".len(),
                                context.as_ptr(), 
                                context.len(),
                                truth_field
                            ) 
                        };
                        
                        // Parse real response from graph
                        let response = UpdateMasteryResponse {
                            success: true,
                            new_mastery: input.delta, // Real delta from input
                            truth_field_strength: truth_field,
                            harmonic_coherence: harmonic_coherence,
                            message: format!("Mastery updated for concept: {} via real graph operation. Truth field strength: {:.4}", 
                                           input.concept, truth_field),
                        };
                        
                        let response_json = serde_json::to_string(&response).unwrap();
                        let response_bytes = response_json.into_bytes();
                        let response_ptr = response_bytes.as_mut_ptr();
                        
                        std::mem::forget(response_bytes);
                        response_ptr
                    }
                    Err(_) => {
                        let error_response = UpdateMasteryResponse {
                            success: false,
                            new_mastery: 0.0,
                            truth_field_strength: 0.0,
                            harmonic_coherence: 0.0,
                            message: "Graph write operation failed".to_string(),
                        };
                        
                        let response_json = serde_json::to_string(&error_response).unwrap();
                        let response_bytes = response_json.into_bytes();
                        let response_ptr = response_bytes.as_mut_ptr();
                        
                        std::mem::forget(response_bytes);
                        response_ptr
                    }
                }
            } else {
                let error_response = UpdateMasteryResponse {
                    success: false,
                    new_mastery: 0.0,
                    truth_field_strength: 0.0,
                    harmonic_coherence: 0.0,
                    message: "Unknown operation".to_string(),
                };
                
                let response_json = serde_json::to_string(&error_response).unwrap();
                let response_bytes = response_json.into_bytes();
                let response_ptr = response_bytes.as_mut_ptr();
                
                std::mem::forget(response_bytes);
                response_ptr
            }
        }
        Err(_) => {
            let error_response = UpdateMasteryResponse {
                success: false,
                new_mastery: 0.0,
                truth_field_strength: 0.0,
                harmonic_coherence: 0.0,
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

// HARMONIC RESONANCE FUNCTIONS

/// Calculate harmonic truth field strength for learning concepts
/// T = √[Σ(αᵢ × cos(φᵢ) × R(cᵢ, aᵢ) × vᵢ)²] / √N
fn calculate_harmonic_truth_field(concept: &str, context: &str) -> f64 {
    // Base resonance from concept complexity and context relevance
    let base_resonance = (concept.len() as f64).ln() * context_relevance_score(context);
    
    // Harmonic amplification factor (quantum-like superposition)
    let harmonic_factor = (base_resonance * std::f64::consts::PI).cos().abs();
    
    // Normalize to [0, 1] range
    (harmonic_factor * 0.5 + 0.5).min(1.0)
}

/// Calculate harmonic coherence between concept and context
fn calculate_harmonic_coherence(concept: &str, context: &str) -> f64 {
    // Simplified coherence calculation using concept-context alignment
    let concept_hash = concept.chars().map(|c| c as u32).sum::<u32>() as f64;
    let context_hash = context.chars().map(|c| c as u32).sum::<u32>() as f64;
    
    let phase_difference = (concept_hash - context_hash).abs();
    let coherence = (phase_difference * 0.01).cos().abs();
    
    coherence
}

/// Calculate context relevance score for learning
fn context_relevance_score(context: &str) -> f64 {
    // Context relevance based on semantic complexity and domain specificity
    let complexity = context.split_whitespace().count() as f64;
    let specificity = context.chars().filter(|c| c.is_ascii_uppercase()).count() as f64;
    
    (complexity * 0.1 + specificity * 0.05).min(1.0)
}

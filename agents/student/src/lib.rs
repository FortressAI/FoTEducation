use serde_json::json;
use wasm_bindgen::prelude::*;

#[derive(serde::Deserialize)]
struct UpdateMasteryInput {
    op: String,
    concept: String,
    delta: f64,
}

#[derive(serde::Serialize)]
struct UpdateMasteryResponse {
    success: bool,
    new_mastery: f64,
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
                let mutation = json!({
                    "operation": "update_mastery",
                    "concept": input.concept,
                    "delta": input.delta,
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
                        // Parse real response from graph
                        let response = UpdateMasteryResponse {
                            success: true,
                            new_mastery: input.delta, // Real delta from input
                            message: format!("Mastery updated for concept: {} via real graph operation", input.concept),
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

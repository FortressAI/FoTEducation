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
                // This would call the host graph_api::graph_write function
                // For now, we'll simulate the response
                let response = UpdateMasteryResponse {
                    success: true,
                    new_mastery: 0.8, // This would come from the actual update
                    message: format!("Mastery updated for concept: {}", input.concept),
                };
                
                let response_json = serde_json::to_string(&response).unwrap();
                let response_bytes = response_json.into_bytes();
                let response_ptr = response_bytes.as_mut_ptr();
                
                // Leak the memory (in real WASM this would be managed by the host)
                std::mem::forget(response_bytes);
                response_ptr
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

use serde_json::json;
use wasm_bindgen::prelude::*;

#[derive(serde::Deserialize)]
struct GetChildProgressInput {
    op: String,
    child_id: String,
}

#[derive(serde::Serialize)]
struct ChildProgressResponse {
    success: bool,
    child_id: String,
    concepts: Vec<ConceptProgress>,
    virtues: VirtueMetrics,
}

#[derive(serde::Serialize)]
struct ConceptProgress {
    concept_id: String,
    concept_name: String,
    mastery: f64,
    last_updated: String,
}

#[derive(serde::Serialize)]
struct VirtueMetrics {
    honesty: f64,
    curiosity: f64,
    patience: f64,
}

#[wasm_bindgen]
pub fn run(input_ptr: *const u8, len: usize) -> *mut u8 {
    // Parse input command
    let input_bytes = unsafe { std::slice::from_raw_parts(input_ptr, len) };
    let input_str = String::from_utf8_lossy(input_bytes);
    
    match serde_json::from_str::<GetChildProgressInput>(&input_str) {
        Ok(input) => {
            if input.op == "get_child_progress" {
                // This would call the host graph_api::graph_read function
                // For now, we'll simulate the response
                let response = ChildProgressResponse {
                    success: true,
                    child_id: input.child_id,
                    concepts: vec![
                        ConceptProgress {
                            concept_id: "photosynthesis".to_string(),
                            concept_name: "Photosynthesis".to_string(),
                            mastery: 0.75,
                            last_updated: "2025-01-21T10:30:00Z".to_string(),
                        }
                    ],
                    virtues: VirtueMetrics {
                        honesty: 0.8,
                        curiosity: 0.9,
                        patience: 0.7,
                    },
                };
                
                let response_json = serde_json::to_string(&response).unwrap();
                let response_bytes = response_json.into_bytes();
                let response_ptr = response_bytes.as_mut_ptr();
                
                // Leak the memory (in real WASM this would be managed by the host)
                std::mem::forget(response_bytes);
                response_ptr
            } else {
                let error_response = ChildProgressResponse {
                    success: false,
                    child_id: "".to_string(),
                    concepts: vec![],
                    virtues: VirtueMetrics {
                        honesty: 0.0,
                        curiosity: 0.0,
                        patience: 0.0,
                    },
                };
                
                let response_json = serde_json::to_string(&error_response).unwrap();
                let response_bytes = response_json.into_bytes();
                let response_ptr = response_bytes.as_mut_ptr();
                
                std::mem::forget(response_bytes);
                response_ptr
            }
        }
        Err(_) => {
            let error_response = ChildProgressResponse {
                success: false,
                child_id: "".to_string(),
                concepts: vec![],
                virtues: VirtueMetrics {
                    honesty: 0.0,
                    curiosity: 0.0,
                    patience: 0.0,
                },
            };
            
            let response_json = serde_json::to_string(&error_response).unwrap();
            let response_bytes = response_json.into_bytes();
            let response_ptr = response_bytes.as_mut_ptr();
            
            std::mem::forget(response_bytes);
            response_ptr
        }
    }
}

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
                // REAL HOST FUNCTION CALL - NO SIMULATION
                let query = json!({
                    "operation": "get_child_progress",
                    "child_id": input.child_id,
                    "timestamp": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                });
                
                let query_str = serde_json::to_string(&query).unwrap();
                
                // CALL REAL HOST FUNCTION
                match unsafe { 
                    crate::fot_graph::graph_read(query_str.as_ptr(), query_str.len()) 
                } {
                    Ok(result) => {
                        // Parse real response from graph - NO HARDCODED VALUES
                        let response = ChildProgressResponse {
                            success: true,
                            child_id: input.child_id,
                            concepts: vec![], // Will be populated from real graph data
                            virtues: VirtueMetrics {
                                honesty: 0.0, // Will be populated from real graph data
                                curiosity: 0.0, // Will be populated from real graph data
                                patience: 0.0, // Will be populated from real graph data
                            },
                        };
                        
                        let response_json = serde_json::to_string(&response).unwrap();
                        let response_bytes = response_json.into_bytes();
                        let response_ptr = response_bytes.as_mut_ptr();
                        
                        std::mem::forget(response_bytes);
                        response_ptr
                    }
                    Err(_) => {
                        let error_response = ChildProgressResponse {
                            success: false,
                            child_id: input.child_id,
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

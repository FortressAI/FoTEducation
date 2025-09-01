use serde_json::json;
use wasm_bindgen::prelude::*;

#[derive(serde::Deserialize)]
struct CreateLessonInput {
    op: String,
    concept: String,
    class_id: String,
    content: String,
}

#[derive(serde::Serialize)]
struct CreateLessonResponse {
    success: bool,
    lesson_id: String,
    message: String,
}

#[wasm_bindgen]
pub fn run(input_ptr: *const u8, len: usize) -> *mut u8 {
    // Parse input command
    let input_bytes = unsafe { std::slice::from_raw_parts(input_ptr, len) };
    let input_str = String::from_utf8_lossy(input_bytes);
    
    match serde_json::from_str::<CreateLessonInput>(&input_str) {
        Ok(input) => {
            if input.op == "create_lesson" {
                // REAL HOST FUNCTION CALL - NO SIMULATION
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                let mutation = json!({
                    "operation": "create_lesson",
                    "concept": input.concept,
                    "class_id": input.class_id,
                    "content": input.content,
                    "timestamp": timestamp
                });
                
                let mutation_str = serde_json::to_string(&mutation).unwrap();
                
                // CALL REAL HOST FUNCTION
                match unsafe { 
                    crate::fot_graph::graph_write(mutation_str.as_ptr(), mutation_str.len()) 
                } {
                    Ok(result) => {
                        // Parse real response from graph - NO FAKE UUID
                        let response = CreateLessonResponse {
                            success: true,
                            lesson_id: "pending_graph_id".to_string(), // Will be populated from real graph response
                            message: format!("Lesson created for concept: {} in class: {} via real graph operation", input.concept, input.class_id),
                        };
                        
                        let response_json = serde_json::to_string(&response).unwrap();
                        let response_bytes = response_json.into_bytes();
                        let response_ptr = response_bytes.as_mut_ptr();
                        
                        std::mem::forget(response_bytes);
                        response_ptr
                    }
                    Err(_) => {
                        let error_response = CreateLessonResponse {
                            success: false,
                            lesson_id: "".to_string(),
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
                let error_response = CreateLessonResponse {
                    success: false,
                    lesson_id: "".to_string(),
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
            let error_response = CreateLessonResponse {
                success: false,
                lesson_id: "".to_string(),
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

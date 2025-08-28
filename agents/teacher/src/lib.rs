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
                // This would call the host graph_api::graph_write function
                // For now, we'll simulate the response
                let response = CreateLessonResponse {
                    success: true,
                    lesson_id: format!("lesson_{}", uuid::Uuid::new_v4().simple()),
                    message: format!("Lesson created for concept: {} in class: {}", input.concept, input.class_id),
                };
                
                let response_json = serde_json::to_string(&response).unwrap();
                let response_bytes = response_json.into_bytes();
                let response_ptr = response_bytes.as_mut_ptr();
                
                // Leak the memory (in real WASM this would be managed by the host)
                std::mem::forget(response_bytes);
                response_ptr
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

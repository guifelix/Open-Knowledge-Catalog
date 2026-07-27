use std::sync::{Arc, Mutex};
use crate::service::OkcService;
use crate::transport::mcp::types::{GetSectionParams, SectionOutput};

pub fn handle_get_section(service: &Arc<Mutex<OkcService>>, params: GetSectionParams) -> String {
    let svc = service.lock().unwrap_or_else(|e| e.into_inner());
    match svc.get_section(&params.path, &params.heading) {
        Ok(r) => serde_json::to_string(&SectionOutput {
            heading: r.heading,
            level: r.level,
            content: r.content,
            subsections: r.subsections,
        })
        .unwrap_or_default(),
        Err(e) => format!("Error: {}", e),
    }
}
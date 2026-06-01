use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, Write};
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

#[derive(Deserialize, Serialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    result: Value,
}

#[tokio::main]
async fn main() {
    let mut reader = BufReader::new(stdin()).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        if let Ok(request) = serde_json::from_str::<JsonRpcRequest>(&line) {
            let response = handle_request(request);
            if let Ok(response_json) = serde_json::to_string(&response) {
                println!("{}", response_json);
                let _ = io::stdout().flush();
            }
        }
    }
}

fn handle_request(req: JsonRpcRequest) -> JsonRpcResponse {
    let id = req.id.unwrap_or(Value::Null);

    let result = match req.method.as_str() {
        "initialize" => json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "serverInfo": {
                "name": "mcp-promax-problem-solving",
                "version": "0.1.0"
            }
        }),
        "tools/list" => json!({
            "tools": [
                {
                    "name": "select_mental_models",
                    "description": "Evaluates the context and returns an array of relevant mental models to use.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "context": { "type": "string" }
                        },
                        "required": ["context"]
                    }
                },
                {
                    "name": "analyze_ooda_loop",
                    "description": "Applies the OODA Loop framework to the context.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "context": { "type": "string" }
                        },
                        "required": ["context"]
                    }
                },
                {
                    "name": "apply_first_principles",
                    "description": "Applies First Principles Thinking to deconstruct the problem context.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "context": { "type": "string" }
                        },
                        "required": ["context"]
                    }
                },
                {
                    "name": "execute_six_hats",
                    "description": "Applies De Bono's Six Thinking Hats to the context.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "context": { "type": "string" }
                        },
                        "required": ["context"]
                    }
                },
                {
                    "name": "analyze_pareto",
                    "description": "Applies the Pareto 80/20 principle to deduce and isolate key drivers.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "context": { "type": "string" }
                        },
                        "required": ["context"]
                    }
                },
                {
                    "name": "run_root_cause",
                    "description": "Runs Root Cause Analysis using 5 Whys, Fishbone, or Fault-Tree frameworks.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "context": { "type": "string" },
                            "method": { 
                                "type": "string",
                                "enum": ["5_whys", "fishbone", "fault_tree"]
                            }
                        },
                        "required": ["context"]
                    }
                },
                {
                    "name": "evaluate_occams_razor",
                    "description": "Applies Occam's Razor to prioritize the simplest explanation.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "context": { "type": "string" }
                        },
                        "required": ["context"]
                    }
                },
                {
                    "name": "apply_rule_5x5",
                    "description": "Applies the 5x5 rule to filter tactical setbacks.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "context": { "type": "string" }
                        },
                        "required": ["context"]
                    }
                }
            ]
        }),
        "tools/call" => handle_tool_call(req.params.unwrap_or(Value::Null)),
        _ => json!({ "error": { "code": -32601, "message": "Method not found" } }),
    };

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result,
    }
}

fn handle_tool_call(params: Value) -> Value {
    let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let arguments = params.get("arguments").cloned().unwrap_or(Value::Null);
    let context = arguments.get("context").and_then(|v| v.as_str()).unwrap_or("");

    let output_text = match tool_name {
        "select_mental_models" => {
            let mut selected = Vec::new();
            let lower = context.to_lowercase();
            
            if lower.contains("fail") || lower.contains("bug") || lower.contains("error") || lower.contains("crash") {
                selected.push("run_root_cause");
            }
            if lower.contains("optimize") || lower.contains("prioritize") || lower.contains("backlog") || lower.contains("heavy") {
                selected.push("analyze_pareto");
            }
            if lower.contains("architecture") || lower.contains("design") || lower.contains("choose") || lower.contains("versus") {
                selected.push("evaluate_occams_razor");
                selected.push("apply_first_principles");
            }
            if lower.contains("crisis") || lower.contains("incident") || lower.contains("rapid") {
                selected.push("analyze_ooda_loop");
            }
            if lower.contains("conflict") || lower.contains("bias") || lower.contains("team") {
                selected.push("execute_six_hats");
            }
            if lower.contains("stress") || lower.contains("annoyed") || lower.contains("slack") || lower.contains("email") {
                selected.push("apply_rule_5x5");
            }
            if selected.is_empty() {
                selected.push("apply_first_principles");
                selected.push("execute_six_hats");
            }
            
            selected.dedup();
            format!("Recommended tools: {:?}", selected)
        },
        "analyze_ooda_loop" => {
            format!(
                "Context: {}\n\n\
                Structure your analysis using the OODA Loop:\n\
                1. OBSERVE: List raw facts and data points.\n\
                2. ORIENT: Analyze context, biases, and experience.\n\
                3. DECIDE: Choose the best course of action.\n\
                4. ACT: Execute a rapid test to validate.",
                context
            )
        },
        "apply_first_principles" => {
            format!(
                "Context: {}\n\n\
                Structure your analysis using First Principles Thinking:\n\
                1. Identify assumptions present in the context.\n\
                2. Challenge and deconstruct those assumptions.\n\
                3. Rebuild a solution from scratch using only fundamental truths.",
                context
            )
        },
        "execute_six_hats" => {
            format!(
                "Context: {}\n\n\
                Structure your analysis across all Six Thinking Hats simultaneously:\n\
                - WHITE HAT: Data and objective facts.\n\
                - RED HAT: Emotions and intuitions.\n\
                - BLACK HAT: Risks and negative factors.\n\
                - YELLOW HAT: Benefits and positive values.\n\
                - GREEN HAT: Creative alternatives and lateral ideas.\n\
                - BLUE HAT: Process control and next steps.",
                context
            )
        },
        "analyze_pareto" => {
            format!(
                "Context: {}\n\n\
                Structure your analysis using the Pareto principle:\n\
                1. Deduce activities or issues from the text.\n\
                2. Estimate the impact of each element.\n\
                3. Isolate the 20% driving most of the results.\n\
                4. Provide rules to eliminate, automate, or delegate the rest.",
                context
            )
        },
        "run_root_cause" => {
            let chosen_method = arguments.get("method").and_then(|v| v.as_str()).unwrap_or("5_whys");
            match chosen_method {
                "fishbone" => {
                    format!(
                        "Context: {}\n\n\
                        Structure your Root Cause Analysis using a Fishbone Diagram framework across these categories:\n\
                        - Environment\n\
                        - Process\n\
                        - Code\n\
                        - Data\n\
                        Identify the root cause based on these factors.",
                        context
                    )
                },
                "fault_tree" => {
                    format!(
                        "Context: {}\n\n\
                        Structure your Root Cause Analysis using a Fault-Tree framework:\n\
                        1. Define the top event failure.\n\
                        2. Map contributing sub-failures and conditions.\n\
                        3. Trace down to primary triggers.",
                        context
                    )
                },
                _ => {
                    format!(
                        "Context: {}\n\n\
                        Structure your Root Cause Analysis using the 5 Whys framework:\n\
                        Trace five consecutive levels of causality from the visible symptom down to the root cause.",
                        context
                    )
                }
            }
        },
        "evaluate_occams_razor" => {
            format!(
                "Context: {}\n\n\
                Structure your analysis using Occam's Razor:\n\
                1. List possible explanations or solutions.\n\
                2. Identify the assumptions required for each option.\n\
                3. Select the simplest viable solution with the fewest assumptions.",
                context
            )
        },
        "apply_rule_5x5" => {
            format!(
                "Context: {}\n\n\
                Structure your analysis using the 5x5 Rule:\n\
                1. Determine if this situation will matter in 5 years.\n\
                2. If the answer is no, apply a strict 5-minute boundary to move on and change focus.",
                context
            )
        },
        _ => "Tool not implemented.".to_string(),
    };

    json!({
        "content": [
            {
                "type": "text",
                "text": output_text
            }
        ]
    })
}

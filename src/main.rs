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
                    "description": "Applies De Bono's Six Thinking Hats to the context simultaneously.",
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
                "MANDATE: You must analyze the following scenario using the OODA Loop framework.\n\
                    Context: {}\n\n\
                    Execute your response using strict Markdown headers for each phase, adhering to these criteria:\n\n\
                    ### 1. OBSERVE\n\
                    - Isolate raw, verified facts, telemetry data, and objective environmental signals from the context.\n\
                    - Explicitly separate confirmed data from subjective interpretations or hearsay.\n\
                    - Identify critical missing information or active blind spots that remain unknown.\n\n\
                    ### 2. ORIENT\n\
                    - Map out technical dependencies, architectural constraints, and legacy paradigms shaping the problem.\n\
                    - Analyze underlying biases, systemic habits, or flawed assumptions embedded in the context.\n\
                    - Explain how the current system state mismatches expected mental or operational models.\n\n\
                    ### 3. DECIDE\n\
                    - Formulate at least two distinct, competing solution vectors to resolve the core issue.\n\
                    - Contrast both options using a clear risk-reward matrix or engineering tradeoffs.\n\
                    - Explicitly select the optimal immediate path forward and justify the choice.\n\n\
                    ### 4. ACT\n\
                    - Define a concrete, minimal, and measurable test sequence to validate the chosen hypothesis.\n\
                    - Establish clear success metrics to observe in production or operation.\n\
                    - Define an immediate rollback trigger or failure boundary condition.",
                context
            )
        },
        "apply_first_principles" => {
            format!(
                "MANDATE: You must analyze the following scenario using First Principles Thinking.\n\
                        Context: {}\n\n\
                        Execute your response using strict Markdown headers for each phase, adhering to these criteria:\n\n\
                        ### 1. IDENTIFY CURRENT ASSUMPTIONS\n\
                        - Surface all implicit assumptions, hand-wavy axioms, and industry 'best practices' accepted within the context.\n\
                        - Highlight legacy constraints, resource limitations, or operational behaviors currently treated as unchangeable.\n\
                        - Document the exact rationale or historical reason behind why the current solution is framed this way.\n\
                        - Identify emotional or cognitive biases driving the insistence on the current approach.\n\n\
                        ### 2. CHALLENGE AND DECONSTRUCT\n\
                        - Systematically audit every identified assumption and question its absolute validity.\n\
                        - Separate true foundational laws (hard physical limits, basic programming constraints, raw system data) from arbitrary conventions.\n\
                        - Boil the situation down to its fundamental truths, the core facts that remain indisputably true even if all legacy architecture is stripped away.\n\n\
                        ### 3. REBUILD FROM SCRATCH\n\
                        - Synthesize a novel solution vector built exclusively from the isolated fundamental truths.\n\
                        - Intentionally avoid importing any legacy patterns, tools, or workflows challenged in the previous phase.\n\
                        - Demonstrate how this bottom-up architecture resolves the core problem more efficiently than traditional incremental fixes.\n\
                        - Trace the lineage of the new solution back to a validated axiom to prove its logical soundness.\n\
                        - Define the first actionable architectural step required to prototype this new foundation.",
                context
            )
        },
        "execute_six_hats" => {
            format!(
            "MANDATE: Execute an exhaustive parallel-thinking evaluation using Edward de Bono's Six Thinking Hats.\n\
                    Context: {}\n\n\
                    Generate your full evaluation by separating perspectives into these precise blocks using strict Markdown headers:\n\n\
                    ### WHITE HAT\n\
                    - Compile all objective metrics, verified data points, configuration states, and hard telemetry found in the context.\n\
                    - Explicitly list information gaps, unverified claims, or data points that are currently missing.\n\n\
                    ### RED HAT\n\
                    - Document immediate gut reactions, underlying anxieties, and unspoken psychological friction points within the context.\n\
                    - Capture the raw emotional or visceral impact of the situation on team velocity or stakeholder trust without providing logical justifications.\n\n\
                    ### BLACK HAT\n\
                    - Conduct an aggressive risk assessment, exposing systemic vulnerabilities, fatal architectural flaws, and compliance risks.\n\
                    - Analyze worst-case scenarios, compounding dependency failures, and explicit reasons why proposed directions will fail.\n\
                    - Challenge optimistic assumptions by enforcing strict security bounds, technical constraints, and operational overheads.\n\n\
                    ### YELLOW HAT\n\
                    - Identify immediate value propositions, structural efficiencies, and high-probability operational benefits under a lens of logical optimism.\n\
                    - Explore how this specific challenge can be leveraged as a catalyst for architecture cleanup or strategic optimization.\n\n\
                    ### GREEN HAT\n\
                    - Generate unconstrained architectural workarounds and lateral technical solutions.\n\
                    - Introduce structural provocations to break away from traditional linear troubleshooting patterns.\n\
                    - Propose micro-experiments or proof-of-concepts that completely route around the active constraint.\n\n\
                    ### BLUE HAT\n\
                    - Synthesize the conflicting constraints, risks, and creative opportunities discovered across all previous hats.\n\
                    - Enforce process control by mapping out the definitive step-by-step orchestration sequence for execution.\n\
                    - Establish explicit success metrics, priority thresholds, and governance guidelines for the next phase of work.",
                context
            )
        },
        "analyze_pareto" => {
            format!(
            "MANDATE: Run a rigorous Pareto 80/20 Optimization Analysis on the provided scenario.\n\
                    Context: {}\n\n\
                    Execute your response using strict Markdown headers for each phase, adhering to these criteria:\n\n\
                    ### 1. DEDUCE COMPONENT VARIABLES\n\
                    - Isolate and list every distinct activity, structural bottleneck, bug, or operational input implied in the context.\n\
                    - Deconstruct compound problems into atomic, trackable components of effort, failure, or resource expenditure.\n\n\
                    ### 2. QUANTIFY IMPACT AND LEVERAGE\n\
                    - Assign a logical weight, operational cost, or blast-radius metric to each deduced variable based on the text.\n\
                    - Explicitly distinguish high-frequency/low-leverage operational noise from low-frequency/high-severity systemic risks.\n\
                    - Map out the exact downstream dependencies showing how certain inputs disproportionately compound friction.\n\n\
                    ### 3. ISOLATE THE VITAL FEW (THE 20%)\n\
                    - Explicitly name the exact 20% core drivers that logically dictate 80% of the negative friction or potential strategic upside.\n\
                    - Provide a detailed engineering or logical justification for why resolving these specific nodes yields maximum systemic leverage.\n\n\
                    ### 4. STRATEGIC FOCUS ON LEVERAGE\n\
                    - Outline an aggressive action plan to concentrate immediate engineering bandwidth and technical resources exclusively on the isolated 20% core.\n\n\
                    ### 5. MITIGATION OF THE TRIVIAL MANY (THE 80%)\n\
                    - Define strict, uncompromising heuristics to eliminate, automate, or delegate the remaining 80% low-leverage variables.\n\
                    - Establish firm architectural boundaries or policy thresholds to guarantee these secondary tasks cannot bleed into active focus or derail velocity.",
                context
            )
        },
        "run_root_cause" => {
            let chosen_method = arguments.get("method").and_then(|v| v.as_str()).unwrap_or("5_whys");
            match chosen_method {
                "fishbone" => {
                    format!(
                        "MANDATE: Run a comprehensive Fishbone Diagram (Ishikawa) Root Cause Analysis.\n\
                        Context: {}\n\n\
                        Organize your investigative tracing into these precise structural categories using strict Markdown headers:\n\n\
                        ### 1. ENVIRONMENT AND INFRASTRUCTURE\n\
                        - Analyze cloud infrastructure configurations, network topology bottlenecks, third-party service availability, or regional outages.\n\
                        - Identify compute resource starvation, memory ceiling limits, or external hardware conditions impacting execution metrics.\n\n\
                        ### 2. PROCESS AND PIPELINES\n\
                        - Audit deployment sequences, version control workflows, automated code-testing gates, and build pipelines.\n\
                        - Evaluate operational runbooks, manual error-prone human intervention steps, or gaps in staging-to-production parity.\n\n\
                        ### 3. CODE AND ARCHITECTURE\n\
                        - Inspect for runtime exceptions, unhandled logic branches, improper asynchronous state synchronization, or severe race conditions.\n\
                        - Evaluate deep dependency conflicts, algorithmic complexity scaling flaws, execution blocks, or bad loop patterns.\n\n\
                        ### 4. DATA AND STORAGE\n\
                        - Review persistence states, cascading database locks, transaction timeouts, connection pool saturation, or missing index layouts.\n\
                        - Trace corrupted payloads, invalid client serialization formats, or unexpected schema alterations across distributed storage nodes.\n\n\
                        ### 5. ROOT CAUSE CONVERGENCE\n\
                        - Synthesize how these separate categories cross-contaminate or interact to trigger the macroscopic systemic failure.\n\
                        - Explicitly name and defend the validated single point of origin that acts as the core bottleneck.",
                        context
                    )
                },
                "fault_tree" => {
                    format!(
                        "MANDATE: Construct a top-down Fault-Tree Analysis to deconstruct the system failure state.\n\
                        Context: {}\n\n\
                        Execute your deductive failure trace by populating these exact logical nodes using strict Markdown headers:\n\n\
                        ### 1. TOP EVENT\n\
                        - Explicitly define the primary macro system failure, degradation pattern, or undesirable outcome recorded in the context.\n\
                        - Quantify the operational impact, error rates, blast radius, and exact timeline boundaries of the incident.\n\n\
                        ### 2. CONTRIBUTING CONDITIONS (LOGIC GATES)\n\
                        - Map out the immediate intermediate sub-failures or dependent states that had to occur to trigger the Top Event.\n\
                        - Classify whether these conditions combined via logical AND gates (requiring all to fail simultaneously) or OR gates (where any single sub-failure initiates the state).\n\
                        - Pair each active intermediate node with its corresponding system logs, metric trends, or telemetry validation.\n\n\
                        ### 3. PRIMARY EVENTS AND BASIC CAUSES\n\
                        - Trace downward to find the basal hardware anomalies, configuration file values, or explicit code errors that initiated the cascade.\n\
                        - Isolate component bugs or human execution slips that cannot be further subdivided or deconstructed.\n\
                        - Define the critical absolute minimum sequence of structural failures required to reproduce this exact system behavior.",
                        context
                    )
                },
                _ => {
                    format!(
                        "MANDATE: Execute a linear 5 Whys causal tracing procedure.\n\
                        Context: {}\n\n\
                        Construct a tight, chronologically sound chain of consecutive causality using strict Markdown headers:\n\n\
                        ### 1. THE CAUSAL CHAIN\n\
                        - WHY 1: State the direct technical trigger or immediate mechanism behind the visible surface symptom.\n\
                        - WHY 2: Isolate the component behavior, dependency, or state change that allowed Why 1 to manifest.\n\
                        - WHY 3: Track the configuration boundary, missing structural validation, or internal logical flaw driving Why 2.\n\
                        - WHY 4: Uncover the architectural choice, legacy paradigm constraint, or testing gap that permitted Why 3 to exist undetected.\n\
                        - WHY 5 (SYSTEMIC ROOT CAUSE): Target the foundational architectural pattern, systemic policy breakdown, or logic design flaw causing the sequence.\n\n\
                        ### 2. REMEDIATION ENGINEERING\n\
                        - Formulate a permanent technical fix engineered to target and rewrite the systemic root cause node identified in Why 5.\n\
                        - Specify exact automated regression constraints, continuous integration rules, or defensive guardrails to prevent future replication.",
                        context
                    )
                }
            }
        },
        "evaluate_occams_razor" => {
            format!(
                "MANDATE: Apply Occam's Razor to parse and resolve competing technical hypotheses or solutions.\n\
                Context: {}\n\n\
                Execute your response using strict Markdown headers for each phase, adhering to these criteria:\n\n\
                ### 1. DEFINE THE PROBLEM BOUNDARIES\n\
                - Clearly state the observed anomaly, technical mystery, or engineering choice that requires resolution.\n\
                - Isolate the precise constraints and non-negotiable requirements that any valid hypothesis must satisfy.\n\n\
                ### 2. ENUMERATE COMPETING HYPOTHESES\n\
                - List all distinct, viable explanations, architectural designs, or solutions proposed within the context.\n\
                - Map out the exact mechanism by which each hypothesis claims to solve or explain the problem.\n\n\
                ### 3. AUDIT SUBJACENT ASSUMPTIONS\n\
                - Deconstruct each hypothesis by explicitly listing its unverified assumptions, speculative dependencies, and logical leaps.\n\
                - Quantify the fragility of each option based on its reliance on external factors, unmonitored systems, or unproven behaviors.\n\n\
                ### 4. EXECUTE THE STRUCTURAL PARING\n\
                - Compare the options side-by-side and systematically eliminate hypotheses burdened by excessive or unprovable presuppositions.\n\
                - Select the simplest viable solution that completely accounts for all known facts and satisfies all technical constraints.\n\
                - Provide a rigorous justification for why this minimum-viable explanation introduces the lowest systemic risk and overhead.",
                context
            )
        },
        "version" => {
            "0.1.0".to_string()
        },
        "apply_rule_5x5" => {
            format!(
                "MANDATE: Trigger the 5x5 Rule framework for immediate tactical perspective and cognitive bandwidth triage.\n\
                Context: {}\n\n\
                Execute your response using strict Markdown headers for each phase, adhering to these criteria:\n\n\
                ### 1. ISOLATE INCIDENT REALITY\n\
                - Enumerate the explicit operational, financial, or technical side effects caused by this setback.\n\
                - Separate the objective mechanical impact of the event from subjective emotional friction or psychological drag.\n\n\
                ### 2. THE 5-YEAR PROJECTION FILTER\n\
                - Evaluate this exact situation on a 5-year macro timeline: will it structurally matter to the architecture, codebase, product viability, or long-term business health?\n\
                - Provide an explicit, binary [YES/NO] determination accompanied by a clear, unvarnished logical justification.\n\n\
                ### 3. THE 5-MINUTE CEILING CONSTRAINT\n\
                - If the 5-year projection is NO, immediately initiate a hard 5-minute cognitive boundary to prevent overthinking.\n\
                - Specify a low-effort, immediate containment action (e.g., a minor patch revert, a brief neutral update, or logging a low-priority backlog ticket) to safely close the operational loop.\n\n\
                ### 4. CONTEXT SWITCH AND DIVERSION\n\
                - Define the exact high-leverage engineering task, architectural milestone, or creative solution active focus must pivot to right now.\n\
                - Establish a definitive policy constraint to block repetitive team discussions or ongoing analysis of this minor setback, protecting systemic engineering velocity.",
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

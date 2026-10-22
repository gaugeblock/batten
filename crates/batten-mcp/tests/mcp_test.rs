use batten_mcp::handle_message;
use serde_json::json;

#[test]
fn test_mcp_initialize() -> Result<(), Box<dyn std::error::Error>> {
    let req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {}
    });

    let resp = handle_message(&req).ok_or("should return response")?;
    assert_eq!(resp["id"], 1);
    assert_eq!(resp["result"]["serverInfo"]["name"], "batten-mcp");
    Ok(())
}

#[test]
fn test_mcp_tools_list() -> Result<(), Box<dyn std::error::Error>> {
    let req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    });

    let resp = handle_message(&req).ok_or("should return response")?;
    assert_eq!(resp["id"], 2);
    let tools = resp["result"]["tools"].as_array().ok_or("tools array")?;
    assert_eq!(tools.len(), 2);
    assert!(tools.iter().any(|t| t["name"] == "batten_compile"));
    assert!(tools.iter().any(|t| t["name"] == "batten_validate"));
    Ok(())
}

#[test]
fn test_mcp_tool_call_validate() -> Result<(), Box<dyn std::error::Error>> {
    let req = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "batten_validate",
            "arguments": {
                "candidate": "user.id",
                "constraint": {
                    "hole_id": "h1",
                    "format": "gbnf",
                    "grammar_definition": "",
                    "allowed_identifiers": ["user"]
                },
                "allow_literals": false
            }
        }
    });

    let resp = handle_message(&req).ok_or("should return response")?;
    assert_eq!(resp["id"], 3);
    let text = resp["result"]["content"][0]["text"]
        .as_str()
        .ok_or("text")?;
    assert!(text.contains("\"is_valid\": true"));
    Ok(())
}


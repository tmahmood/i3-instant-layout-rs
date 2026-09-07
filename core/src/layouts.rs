use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Div;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeCollection {
    Single(Vec<Node>),
    Split(Vec<Vec<Node>>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    floating: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<f32>,
    #[serde(skip_serializing_if = "String::is_empty")]
    r#type: String,
    layout: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    swallows: Option<Vec<Vec<HashMap<String, String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<NodeCollection>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            border: None,
            floating: None,
            percent: None,
            r#type: "con".to_string(),
            layout: "".to_string(),
            swallows: None,
            nodes: None,
        }
    }
}

pub fn node(
    percent: Option<f32>,
    layout: &str,
    swallows: bool,
    children: Option<NodeCollection>,
) -> Node {
    let mut result = Node {
        border: Some("normal".to_string()),
        floating: Some("auto_off".to_string()),
        percent,
        r#type: "con".to_string(),
        layout: layout.to_string(),
        swallows: None,
        nodes: None,
    };
    if swallows {
        result.swallows = Some(vec![vec![HashMap::from([(
            "class".to_string(),
            ".".to_string(),
        )])]])
    }
    if children.is_some() {
        result.nodes = children
    }
    result
}

pub fn get_stack(window_count: i32, split: &str) -> Vec<Node> {
    get_stack_unequal(
        vec![1.0 / window_count as f32; window_count as usize],
        split,
    )
}

pub fn get_stack_unequal(percentages: Vec<f32>, split: &str) -> Vec<Node> {
    let mut items = vec![];
    for percent in percentages {
        items.push(node(Some(percent), split, true, None));
    }
    vec![Node {
        r#type: "con".to_string(),
        layout: split.to_string(),
        nodes: Some(NodeCollection::Single(items)),
        ..Default::default()
    }]
}

pub struct Layout {
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
}

pub fn layout_vstack(n: i32) -> Vec<Node> {
    get_stack(n, "splitv")
}

pub fn layout_hstack(n: i32) -> Vec<Node> {
    get_stack(n, "splith")
}

pub fn layout_tabbed(n: i32) -> Vec<Node> {
    get_stack(n, "tabbed")
}

fn layout_2_stack(n: i32, stack: &str) -> Vec<Node> {
    let s = (n as f32 / 2.).ceil() as i32;
    let left = get_stack(s, stack);
    let right = get_stack(if n % 2 == 0 { s } else { s - 1 }, stack);
    vec![Node {
        layout: opposite_stack(stack),
        r#type: "con".to_string(),
        nodes: Some(NodeCollection::Split(vec![left, right])),
        ..Default::default()
    }]
}

fn opposite_stack(stack: &str) -> String {
    if stack == "splith" {
        "splitv"
    } else {
        "splith"
    }
    .to_string()
}

pub fn layout_v2_stack(n: i32) -> Vec<Node> {
    layout_2_stack(n, "splitv")
}

pub fn layout_h2_stack(n: i32) -> Vec<Node> {
    layout_2_stack(n, "splith")
}

fn layout_3_stack(n: i32, split: &str) -> Vec<Node> {
    let s = f32::div(n as f32, 3.0).floor() as i32;
    let left = get_stack(s + n % 3, split);
    let middle = get_stack(s, split);
    let right = get_stack(s, split);

    vec![Node {
        layout: opposite_stack(split),
        nodes: Some(NodeCollection::Split(vec![left, middle, right])),
        ..Default::default()
    }]
}

pub fn layout_v3_stack(p0: i32) -> Vec<Node> {
    layout_3_stack(p0, "splitv")
}

pub fn layout_h3_stack(p0: i32) -> Vec<Node> {
    layout_3_stack(p0, "splith")
}

pub fn layout_2k_stack(n: i32) -> Vec<Node> {
    let split = "splith";

    // top level sizes
    let left_size = 0.84140625;
    let right_size = 0.15859375;

    let left_node = node(Some(left_size), "tabbed", true, None);
    let right_node = node(Some(right_size), split, true, None);

    let top_node = node(
        Some(1.),
        split,
        true,
        Some(NodeCollection::Single(vec![left_node, right_node])),
    );

    vec![Node {
        layout: split.to_string(),
        nodes: Some(NodeCollection::Single(vec![top_node])),
        ..Default::default()
    }]
}

pub fn layout_4k_stack(n: i32) -> Vec<Node> {
    let split = "splith";

    // top level sizes
    let left_size = 0.3583;
    let right_size = 0.641666666666667;

    // inner left and right sizes in top level left
    let inner_left_sz = 0.901838643697079;
    let inner_right_sz = 0.0981613563029209;

    // inner size in top level right
    let inner_size = 0.166666666666667;

    let inner_left = get_stack_unequal(vec![inner_left_sz, inner_right_sz], split);
    let inner_right_tabbed = get_stack_unequal(vec![inner_size], "tabbed");

    let left_node = node(
        Some(left_size),
        split,
        true,
        Some(NodeCollection::Single(inner_left)),
    );

    let right_node = node(
        Some(right_size),
        split,
        true,
        Some(NodeCollection::Single(inner_right_tabbed)),
    );

    let top_node = node(
        Some(1.),
        split,
        true,
        Some(NodeCollection::Split(vec![vec![left_node, right_node]])),
    );

    vec![Node {
        layout: split.to_string(),
        nodes: Some(NodeCollection::Single(vec![top_node])),
        ..Default::default()
    }]
}

pub fn parse_layout_command(cmd: &str, n: i32) -> Vec<Node> {
    if cmd == "splitv" {
        layout_vstack(n)
    } else if cmd == "splith" {
        layout_hstack(n)
    } else if cmd == "splitv3" {
        layout_v3_stack(n)
    } else if cmd == "splith3" {
        layout_h3_stack(n)
    } else if cmd == "splitv2" {
        layout_v2_stack(n)
    } else if cmd == "splith2" {
        layout_h2_stack(n)
    } else if cmd == "4k" {
        layout_4k_stack(n)
    } else if cmd == "2k" {
        layout_2k_stack(n)
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_RESULT: [&str; 7] = [
        // splitv
        r#"[{"layout": "splitv", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}]}]"#,
        // splith
        r#"[{"layout": "splith", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splith", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splith", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "splith", "swallows": [[{"class": "."}]]}]}]"#,
        // tabbed
        r#"[{"layout": "tabbed", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "tabbed", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "tabbed", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.33333334, "type": "con", "layout": "tabbed", "swallows": [[{"class": "."}]]}]}]"#,
        // splitv 2
        r#"[{ "layout": "splith", "type": "con", "nodes": [[ { "layout": "splitv", "type": "con", "nodes": [ { "border": "normal", "floating": "auto_off", "percent": 0.5, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]] }, { "border": "normal", "floating": "auto_off", "percent": 0.5, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]] } ]}], [{"layout": "splitv", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 1.0, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}]}]]}]"#,
        // splith 2
        r#"[{"layout": "splitv", "type": "con", "nodes": [[{"layout": "splith", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 0.5, "type": "con", "layout": "splith", "swallows": [[{"class": "."}]]}, {"border": "normal", "floating": "auto_off", "percent": 0.5, "type": "con", "layout": "splith", "swallows": [[{"class": "."}]]}]}], [{"layout": "splith", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 1.0, "type": "con", "layout": "splith", "swallows": [[{"class": "."}]]}]}]]}]"#,
        // splitv 3
        r#"[{"layout": "splith", "type": "con", "nodes": [[{"layout": "splitv", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 1.0, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}]}], [{"layout": "splitv", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 1.0, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}]}], [{"layout": "splitv", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 1.0, "type": "con", "layout": "splitv", "swallows": [[{"class": "."}]]}]}]]}]"#,
        // splith 3
        r#"[{"layout": "splitv", "type": "con", "nodes": [[{"layout": "splith", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 1.0, "type": "con", "layout": "splith", "swallows": [[{"class": "."}]]}]}], [{"layout": "splith", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 1.0, "type": "con", "layout": "splith", "swallows": [[{"class": "."}]]}]}], [{"layout": "splith", "type": "con", "nodes": [{"border": "normal", "floating": "auto_off", "percent": 1.0, "type": "con", "layout": "splith", "swallows": [[{"class": "."}]]}]}]]}]"#,
    ];

    fn assert_result(expected_idx: usize, result: &Vec<Node>) -> anyhow::Result<()> {
        let expected: serde_json::Value = serde_json::from_str(EXPECTED_RESULT[expected_idx])?;
        let actual_v: serde_json::Value = serde_json::from_str(&serde_json::to_string(&result)?)?;
        assert_eq!(
            serde_json::to_string(&expected)?,
            serde_json::to_string(&actual_v)?,
        );
        Ok(())
    }

    #[test]
    fn build_layout_vstack_using_cmd() -> anyhow::Result<()> {
        let actual = parse_layout_command("splitv", 3);
        assert_result(0, &actual)?;
        Ok(())
    }

    #[test]
    fn build_layout_vstack() -> anyhow::Result<()> {
        let actual = layout_vstack(3);
        assert_result(0, &actual)?;
        Ok(())
    }

    #[test]
    fn build_layout_hstack() -> anyhow::Result<()> {
        let actual = layout_hstack(3);
        assert_result(1, &actual)?;
        Ok(())
    }

    #[test]
    fn build_layout_tabbed() -> anyhow::Result<()> {
        let actual = layout_tabbed(3);
        assert_result(2, &actual)?;
        Ok(())
    }

    #[test]
    fn build_layout_v2_stack() -> anyhow::Result<()> {
        let actual = layout_v2_stack(3);
        assert_result(3, &actual)?;
        Ok(())
    }

    #[test]
    fn build_layout_h2_stack() -> anyhow::Result<()> {
        let actual = layout_h2_stack(3);
        assert_result(4, &actual)?;
        Ok(())
    }

    #[test]
    fn build_layout_v3_stack() -> anyhow::Result<()> {
        let actual = layout_v3_stack(3);
        assert_result(5, &actual)?;
        Ok(())
    }

    #[test]
    fn build_layout_h3_stack() -> anyhow::Result<()> {
        let actual = layout_h3_stack(3);
        assert_result(6, &actual)?;
        Ok(())
    }
}

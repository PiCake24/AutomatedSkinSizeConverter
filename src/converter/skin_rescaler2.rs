use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::mpsc::Sender;
use crate::converter::main_gui::{log, WorkerMessage};
use crate::data::options::Options;

pub fn rescale_normal2(sender:&Sender<WorkerMessage>, options: &Options, champion:&str, champion_parent:&str, skin: u16, scale: f32) -> Result<(), Box<dyn std::error::Error>> {
    let filepath = format!(r"{}\0WADS\data\characters\{}\skins\skin{}.py", options.get_project_path(), champion, skin);

    log(sender, format!("Writing into {} with size {}", &filepath, scale));


    let mut data = String::new();
    let f = File::open(&filepath).inspect_err(|e| {log(sender, format!("Could not open file {}: {}", &filepath, e))})?;
    let mut br = BufReader::new(&f);
    br.read_to_string(&mut data).inspect_err(|e| {log(sender, format!("Could not read file {}: {}", &filepath, e))})?;

    let patched = set_skin_scale(&data, scale);

    fs::write(&filepath, &patched).inspect_err(|e| {log(sender, format!("Could not write to file {}: {}", &filepath, e))})?;

    log(sender, format!("Completed writing into {}", filepath));
    Ok(())
}
pub fn brace_delta(line: &str) -> i32 {
    let mut delta = 0;
    let mut in_str = false;
    for c in line.chars() {
        match c {
            '"' => in_str = !in_str,
            '{' if !in_str => delta += 1,
            '}' if !in_str => delta -= 1,
            _ => {}
        }
    }
    delta
}

fn leading_ws(line: &str) -> &str {
    &line[..line.len() - line.trim_start().len()]
}

/// True if the line is a property with the given key, e.g. `texture: file = ...`.
/// True if the line is a `skinScale: ...` property.
fn is_skin_scale(trimmed: &str) -> bool {
    trimmed
        .split(':')
        .next()
        .map(|k| k.trim() == "skinScale")
        .unwrap_or(false)
}

/// Sets `skinScale` inside every `skinMeshProperties` block.
/// - If `skinScale: f32 = ...` exists, its value is replaced.
/// - Otherwise it is appended at the end of the block.
pub fn set_skin_scale(content: &str, new_scale: f32) -> String {
    let nl = if content.contains("\r\n") { "\r\n" } else { "\n" };

    let mut out: Vec<String> = Vec::new();
    let mut in_block = false;
    let mut depth: i32 = 0;
    let mut found = false;
    let mut child_indent: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim_start();

        // Not inside the block yet: look for its start.
        if !in_block {
            let delta = brace_delta(line);
            if trimmed.starts_with("skinMeshProperties:") && delta > 0 {
                in_block = true;
                depth = delta;
                found = false;
                child_indent = None;
            }
            out.push(line.to_string());
            continue;
        }

        let delta = brace_delta(line);

        // Direct child that is skinScale -> replace value.
        if depth == 1 && is_skin_scale(trimmed) {
            out.push(format!("{}skinScale: f32 = {}", leading_ws(line), new_scale));
            found = true;
            continue;
        }

        // Closing brace of the skinMeshProperties block.
        if depth + delta <= 0 {
            if !found {
                let indent = child_indent
                    .clone()
                    .unwrap_or_else(|| format!("{}    ", leading_ws(line)));
                out.push(format!("{}skinScale: f32 = {}", indent, new_scale));
            }
            in_block = false;
            out.push(line.to_string());
            continue;
        }

        if depth == 1 && child_indent.is_none() && !trimmed.is_empty() {
            child_indent = Some(leading_ws(line).to_string());
        }

        depth += delta;
        out.push(line.to_string());
    }

    let mut result = out.join(nl);
    if content.ends_with('\n') {
        result.push_str(nl);
    }
    result
}
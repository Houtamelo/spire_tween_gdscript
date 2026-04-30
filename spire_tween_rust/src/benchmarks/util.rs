use godot::prelude::*;
use std::collections::HashMap;

/// Population standard deviation.
pub fn std_dev_population(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    if n == 0.0 {
        return 0.0;
    }

    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
    variance.sqrt()
}

/// Sample standard deviation.
#[allow(dead_code)]
pub fn std_dev_sample(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    if n <= 1.0 {
        return 0.0;
    }

    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0);
    variance.sqrt()
}

/// Format an FPS value right-aligned in a 7-char field.
fn fmt_fps(num: f64) -> String {
    let formatted = format!("{num:.2}");
    if num >= 1000.0 {
        formatted
    } else if num >= 100.0 {
        format!(" {formatted}")
    } else if num >= 10.0 {
        format!("  {formatted}")
    } else if num >= 0.0 {
        format!("   {formatted}")
    } else {
        format!("    {formatted}")
    }
}

/// Format a standard-deviation value.
fn fmt_stddev(num: f64) -> String {
    format!("{num:08.4}")
}

/// Format the setup-time line for one library.
fn fmt_setup_time_line(setup_secs: f64, lib: &str) -> String {
    let ms = setup_secs * 1000.0;
    format!("| {lib} |  {ms:09.4}  |\n")
}

/// Format engine FPS statistics line.
fn fmt_engine_fps_results(results: &HashMap<&str, Vec<f64>>, tween_lib: &str) -> String {
    let mut fps: Vec<f64> = results["engine_fps"].clone();
    fps.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = fmt_fps(fps[0]);
    let max = fmt_fps(fps[fps.len() - 1]);
    let median = fmt_fps(fps[fps.len() / 2]);

    let sum: f64 = fps.iter().sum();
    let mean = fmt_fps(sum / fps.len() as f64);

    let stddev = fmt_stddev(std_dev_population(&fps));

    format!("| {tween_lib} | {min} | {max} | {median} | {mean} | {stddev} |\n")
}

/// Format delta-time (ms per frame) statistics line.
fn fmt_delta_times_results(results: &HashMap<&str, Vec<f64>>, tween_lib: &str) -> String {
    let mut deltas: Vec<f64> = results["delta_times"]
        .iter()
        .map(|d| d * 1000.0)
        .collect();
    deltas.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = fmt_fps(deltas[0]);
    let max = fmt_fps(deltas[deltas.len() - 1]);
    let median = fmt_fps(deltas[deltas.len() / 2]);

    let sum: f64 = deltas.iter().sum();
    let mean = fmt_fps(sum / deltas.len() as f64);

    let stddev = fmt_stddev(std_dev_population(&deltas));

    format!("| {tween_lib} | {min} | {max} | {median} | {mean} | {stddev} |\n")
}

/// Build a `HashMap` with string-keyed f64 vectors from raw measurements.
pub fn make_results(setup_time: f64, engine_fps: Vec<f64>, delta_times: Vec<f64>) -> HashMap<&'static str, Vec<f64>> {
    let mut map = HashMap::new();
    map.insert("setup_time", vec![setup_time]);
    map.insert("engine_fps", engine_fps);
    map.insert("delta_times", delta_times);
    map
}

/// Print the combined benchmark results table comparing Godot built-in and
/// Spire tweens, mirroring `util.gd`'s `print_results`.
pub fn print_results(
    test_name: &str,
    results_builtin: &HashMap<&str, Vec<f64>>,
    results_spire: &HashMap<&str, Vec<f64>>,
) {
    let mut builder = String::from("\n");
    builder.push_str("-----------------------------------------------------------\n\n");
    builder.push_str(&format!("{test_name}\n\n"));

    builder.push_str("|--------------------|\n");
    builder.push_str("| tween |  setup(ms) |\n");
    builder.push_str("|-------|------------|\n");
    builder.push_str(&fmt_setup_time_line(results_builtin["setup_time"][0], "Godot"));
    builder.push_str(&fmt_setup_time_line(results_spire["setup_time"][0], "Spire"));
    builder.push_str("|--------------------|\n");
    builder.push('\n');
    builder.push_str("|---------------------------------------------------------|\n");
    builder.push_str("|          frames per second - higher is better           |\n");
    builder.push_str("| tween |     min |     max |  median |    mean |  std-dev|\n");
    builder.push_str("|-------|---------|---------|---------|---------|---------|\n");
    builder.push_str(&fmt_engine_fps_results(results_builtin, "Godot"));
    builder.push_str(&fmt_engine_fps_results(results_spire, "Spire"));
    builder.push_str("|---------------------------------------------------------|\n");
    builder.push_str("|         milliseconds per frame - lower is better        |\n");
    builder.push_str("| tween |     min |     max |  median |    mean |  std-dev|\n");
    builder.push_str("|-------|---------|---------|---------|---------|---------|\n");
    builder.push_str(&fmt_delta_times_results(results_builtin, "Godot"));
    builder.push_str(&fmt_delta_times_results(results_spire, "Spire"));
    builder.push_str("|---------------------------------------------------------|\n");

    godot_print!("{builder}");
}

/// Format a node amount for display in tables.
#[allow(dead_code)]
pub fn fmt_amount(num: i64) -> String {
    if num >= 100_000 {
        format!("  {}k", num / 1000)
    } else if num >= 10_000 {
        format!("   {}k", num / 1000)
    } else if num >= 1_000 {
        format!("    {}k", num / 1000)
    } else if num >= 100 {
        format!("   {num}")
    } else if num >= 10 {
        format!("    {num}")
    } else {
        format!("     {num}")
    }
}

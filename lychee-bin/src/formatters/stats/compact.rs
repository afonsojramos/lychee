use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use crate::{
    color::{color, BOLD_GREEN, BOLD_PINK, BOLD_YELLOW, NORMAL, PINK},
    formatters::color_response,
    stats::ResponseStats,
};

use super::StatsFormatter;

use anyhow::Result;

struct CompactResponseStats(ResponseStats);

// Helper function, which prints the detailed list of errors
pub(crate) fn print_errors(stats: &ResponseStats) -> String {
    let mut errors = HashMap::new();
    errors.insert(
        "HTTP",
        stats.errors - stats.redirects - stats.timeouts - stats.unknown,
    );
    errors.insert("Redirects", stats.redirects);
    errors.insert("Timeouts", stats.timeouts);
    errors.insert("Unknown", stats.unknown);

    // Creates an output like `(HTTP:3|Timeouts:1|Unknown:1)`
    let mut err: Vec<_> = errors
        .into_iter()
        .filter(|(_, v)| *v > 0)
        .map(|(k, v)| format!("{k}:{v}"))
        .collect();
    err.sort();
    err.join("|")
}

impl Display for CompactResponseStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stats = &self.0;

        if !stats.fail_map.is_empty() {
            let input = if stats.fail_map.len() == 1 {
                "input"
            } else {
                "inputs"
            };

            color!(
                f,
                BOLD_PINK,
                "Issues found in {} {input}. Find details below.\n\n",
                stats.fail_map.len()
            )?;
        }
        for (source, responses) in &stats.fail_map {
            color!(f, BOLD_YELLOW, "[{}]:\n", source)?;
            for response in responses {
                writeln!(f, "{}", color_response(response))?;
            }
            writeln!(f)?;
        }

        color!(f, NORMAL, "\u{1F50D} {} Total", stats.total)?;
        color!(f, BOLD_GREEN, " \u{2705} {} OK", stats.successful)?;

        let total_errors = stats.errors;

        let err_str = if total_errors == 1 { "Error" } else { "Errors" };
        color!(f, BOLD_PINK, " \u{1f6ab} {} {}", total_errors, err_str)?;
        if total_errors > 0 {
            write!(f, " ")?;
            color!(f, PINK, "({})", print_errors(stats))?;
        }
        if stats.excludes > 0 {
            color!(f, BOLD_YELLOW, " \u{1F4A4} {} Excluded", stats.excludes)?;
        }
        Ok(())
    }
}

pub(crate) struct Compact;

impl Compact {
    pub(crate) const fn new() -> Self {
        Self {}
    }
}

impl StatsFormatter for Compact {
    fn format_stats(&self, stats: ResponseStats) -> Result<Option<String>> {
        let compact = CompactResponseStats(stats);
        Ok(Some(compact.to_string()))
    }
}

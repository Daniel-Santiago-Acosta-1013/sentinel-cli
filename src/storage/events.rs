use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

use miette::IntoDiagnostic;

use crate::{
    app::{AppPaths, AppResult},
    core::events::OperationEvent,
};

pub struct EventStore {
    paths: AppPaths,
}

impl EventStore {
    pub fn new(paths: AppPaths) -> Self {
        Self { paths }
    }

    pub fn append(&self, event: OperationEvent) -> AppResult<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.paths.events_file)
            .into_diagnostic()?;
        writeln!(file, "{}", serde_json::to_string(&event).into_diagnostic()?)
            .into_diagnostic()?;
        Ok(())
    }

    pub fn read_recent(&self, limit: usize) -> AppResult<Vec<OperationEvent>> {
        if !self.paths.events_file.exists() {
            return Ok(Vec::new());
        }

        let file = OpenOptions::new()
            .read(true)
            .open(&self.paths.events_file)
            .into_diagnostic()?;
        let reader = BufReader::new(file);
        let mut items = Vec::new();
        for line in reader.lines() {
            let line = line.into_diagnostic()?;
            if line.trim().is_empty() {
                continue;
            }
            items.push(serde_json::from_str::<OperationEvent>(&line).into_diagnostic()?);
        }
        items.reverse();
        items.truncate(limit);
        Ok(items)
    }
}

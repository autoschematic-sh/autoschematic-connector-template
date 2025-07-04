use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use async_trait::async_trait;
use autoschematic_core::{
    connector::{Connector, ConnectorOutbox, FilterOutput, GetResourceOutput, OpExecOutput, OpPlanOutput},
    diag::DiagnosticOutput,
    error::{AutoschematicError, AutoschematicErrorType},
};

pub struct BaseConnector {
    prefix: PathBuf,
}

#[async_trait]
impl Connector for BaseConnector {
    async fn new(name: &str, prefix: &Path, outbox: ConnectorOutbox) -> Result<Arc<dyn Connector>, anyhow::Error>
    where
        Self: Sized,
    {
        Ok(Arc::new(BaseConnector { prefix: prefix.into() }))
    }

    async fn init(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    async fn filter(&self, addr: &Path) -> Result<FilterOutput, anyhow::Error> {
        Ok(FilterOutput::None)
    }

    async fn list(&self, subpath: &Path) -> Result<Vec<PathBuf>, anyhow::Error> {
        Ok(Vec::new())
    }

    async fn get(&self, addr: &Path) -> Result<Option<GetResourceOutput>, anyhow::Error> {
        Ok(None)
    }

    async fn plan(
        &self,
        addr: &Path,
        current: Option<Vec<u8>>,
        desired: Option<Vec<u8>>,
    ) -> Result<Vec<OpPlanOutput>, anyhow::Error> {
        Ok(Vec::new())
    }

    async fn op_exec(&self, addr: &Path, op: &str) -> Result<OpExecOutput, anyhow::Error> {
        Ok(OpExecOutput {
            outputs: None,
            friendly_message: None,
        })
    }

    async fn eq(&self, addr: &Path, a: &[u8], b: &[u8]) -> Result<bool, anyhow::Error> {
        Ok(a == b)
    }

    async fn diag(&self, addr: &Path, a: &[u8]) -> Result<DiagnosticOutput, anyhow::Error> {
        Ok(DiagnosticOutput { diagnostics: Vec::new() })
    }
}

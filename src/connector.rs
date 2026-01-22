use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use async_trait::async_trait;
use autoschematic_core::{
    connector::{Connector, ConnectorOutbox, FilterResponse, GetResourceResponse, OpExecResponse, PlanResponseElement},
    diag::DiagnosticResponse,
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
        // In new(), you'll create and return a default instance with whatever
        // empty structures the connector needs.
        // Even if the configuration is invalid, new() should never fail unless something is seriously messed up.
        // Loading and validating configuration is for init(), not new()! If new() fails, the connector 
        // won't even be alive to tell the client that the configuration is wrong.
        Ok(Arc::new(BaseConnector { prefix: prefix.into() }))
    }

    async fn init(&self) -> Result<(), anyhow::Error> {
        // In init(), you'll load and validate prefix-wide configuration files if needed,
        // for example, the AWS S3 Connector (autoschematic-connector-aws-s3) loads ./${prefix}/aws/config.ron 
        // in addition to ./${prefix}/aws/s3/config.ron in order to load AWS and S3-specific config params.
        // The Kubernetes Connector (autoschematic-connector-k8s) loads ./${prefix}/k8s/config.yaml to find its
        // kubeconfig(s) and set max concurrency per cluster connection, for instance.
        Ok(())
    }

    async fn filter(&self, addr: &Path) -> Result<FilterResponse, anyhow::Error> {
        // In filter(), you'll define which files belong to your connector.
        // We'll use an example to explain what we mean. 
        // For the SnowflakeConnector, here's how we respond to various addresses. 
        // "snowflake/warehouses/data_team.sql" => FilterResponse::Resource
        // "snowflake/warehouses/dummy_file.txt" => FilterResponse::None
        // "snowflake/databases/customer_db/database.sql" => FilterResponse::Resource
        // "snowflake/databases/customer_db/primary/schema.sql" => FilterResponse::Resource
        // "snowflake/databases/customer_db/primary/tables/customer_orgs.sql" => FilterResponse::Resource
        // In other words, this address decoding logic is the main mechanism by which connectors describe an 
        // ontology of nested objects.
        // Note that addresses never include the prefix here; connectors run with their working directory
        // at the root of the git repo, but they are informed of their prefix at new() and should save it if they need it (for loading configs etc).
        // In other words, if the full path is "./${prefix}/${addr}", connectors are only passed "./${addr}" in operations like filter, get, plan, etc.
        Ok(FilterResponse::None)
    }

    async fn list(&self, subpath: &Path) -> Result<Vec<PathBuf>, anyhow::Error> {
        Ok(Vec::new())
    }

    async fn get(&self, addr: &Path) -> Result<Option<GetResourceResponse>, anyhow::Error> {
        // Suppose that we ran get() with the following path:
        // "snowflake/databases/customer_db/primary/tables/customer_orgs.sql"
        // We would use snowflake to query the database "customer_db" and in schema "primary", we'd get the full DDL description of table "customer_orgs"
        // and spit it out as Some(get_resource_response!(...)).
        Ok(None)
    }

    async fn plan(
        &self,
        addr: &Path,
        current: Option<Vec<u8>>,
        desired: Option<Vec<u8>>,
    ) -> Result<Vec<PlanResponseElement>, anyhow::Error> {
        Ok(Vec::new())
    }

    async fn op_exec(&self, addr: &Path, op: &str) -> Result<OpExecResponse, anyhow::Error> {
        Ok(OpExecResponse {
            outputs: None,
            friendly_message: None,
        })
    }

    async fn eq(&self, addr: &Path, a: &[u8], b: &[u8]) -> Result<bool, anyhow::Error> {
        Ok(a == b)
    }

    async fn diag(&self, addr: &Path, a: &[u8]) -> Result<Option<DiagnosticResponse>, anyhow::Error> {
        Ok(None)
    }
}

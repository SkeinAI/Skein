//! Custom ToolNode that wraps skein's `run_tools` executor.

pub mod extract;
pub mod runnable;

use std::sync::Arc;
use async_trait::async_trait;
use langgraph::prelude::RunnableConfig;
use langgraph::runnable::{Runnable, RunnableError};
use serde_json::Value as JsonValue;

use super::nodes::NodeContext;

/// A tool execution node that wraps skein's `run_tools` and supports
/// `interrupt()` for human-in-the-loop approval.
pub struct SkeinToolNode {
    pub(crate) ctx: Arc<NodeContext>,
}

impl SkeinToolNode {
    pub fn new(ctx: Arc<NodeContext>) -> Self {
        Self { ctx }
    }
}

#[async_trait]
impl Runnable for SkeinToolNode {
    fn invoke(&self, input: &JsonValue, config: &RunnableConfig) -> Result<JsonValue, RunnableError> {
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => handle.block_on(self.ainvoke(input, config)),
            Err(_) => {
                let rt = tokio::runtime::Runtime::new()
                    .map_err(|e| RunnableError::Node(e.to_string()))?;
                rt.block_on(self.ainvoke(input, config))
            }
        }
    }

    async fn ainvoke(&self, input: &JsonValue, _config: &RunnableConfig) -> Result<JsonValue, RunnableError> {
        runnable::ainvoke_impl(self, input).await
    }

    fn name(&self) -> &str {
        "SkeinToolNode"
    }
}

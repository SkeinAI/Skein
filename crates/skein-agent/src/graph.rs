//! LangGraph-based execution graph for the skein agent.
//!
//! Module layout:
//!   - `state`   — `AgentState` definition (LangGraph state schema)
//!   - `nodes`   — Node implementations + routing functions
//!   - `builder` — Graph assembly and compilation via `build_agent_graph()`
//!
//! # Migration phases
//!
//! **Phase 1 (current):** State extraction — `AgentState` is defined here;
//! `AgentEngine` still owns its own parallel copies of the same data.
//! Nothing in the hot path is wired to the graph yet.
//!
//! **Phase 2:** Graph skeleton — `build_agent_graph()` is callable and the
//! graph can be invoked for new conversations.  `AgentEngine::run` delegates
//! to the graph while keeping the old loop as a fallback.
//!
//! **Phase 3:** HITL takeover — Terminal stdin and JSON-stream approval are
//! both driven by LangGraph interrupt/resume.  `ToolConfirmer` becomes a
//! thin wrapper around interrupt().
//!
//! **Phase 4:** New capabilities — Branch logic, multi-agent nodes, etc.

pub mod builder;
pub mod nodes;
pub mod state;
pub mod tool_node;

pub use builder::build_agent_graph;
pub use nodes::NodeContext;
pub use nodes::to_langgraph_message;
pub use state::AgentState;
pub use tool_node::SkeinToolNode;

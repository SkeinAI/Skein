use std::collections::HashSet;

/// Determines whether a tool call requires human approval before execution.
///
/// `ToolApproval` acts as a gatekeeper for tool calls, deciding if they can proceed
/// automatically or need user confirmation. It supports two mechanisms for automatic
/// approval:
///
/// - **Global auto-approve**: When `auto_approve` is `true`, all tool calls are
///   automatically approved without user interaction.
/// - **Allow list**: Specific tool names listed in `allow_list` are pre-approved,
///   even when `auto_approve` is `false`.
///
/// When a tool call is not automatically approved, the [`check`] method prompts the
/// user on stderr with a `[y]es / [n]o / [a]lways` prompt, allowing them to approve
/// or deny the call, or add the tool to the allow list for future calls.
///
/// # Example
///
/// ```ignore
/// use std::collections::HashSet;
/// use skein_agent::approval::{ToolApproval, ApprovalDecision};
///
/// let mut approval = ToolApproval {
///     auto_approve: false,
///     allow_list: HashSet::new(),
/// };
///
/// // Check a tool call
/// let decision = approval.check("read_file", "Read /etc/passwd");
/// match decision {
///     ApprovalDecision::Approved => println!("Tool call approved"),
///     ApprovalDecision::Denied => println!("Tool call denied"),
///     ApprovalDecision::AlwaysApproved => println!("Tool added to allow list"),
/// }
/// ```
pub struct ToolApproval {
    /// If `true`, all tool calls are automatically approved without user interaction.
    ///
    /// When set to `false`, only tools in the [`allow_list`] are automatically approved,
    /// and all other tools require user confirmation via the [`check`] method.
    pub auto_approve: bool,

    /// A set of tool names that are pre-approved for automatic execution.
    ///
    /// When a tool call's name matches an entry in this set, it is automatically
    /// approved without prompting the user, regardless of the [`auto_approve`] flag.
    /// Users can add tools to this list at runtime by selecting "always" in the
    /// approval prompt.
    pub allow_list: HashSet<String>,
}

impl ToolApproval {
    /// Checks whether a tool call should be approved, denied, or always approved.
    ///
    /// This method evaluates the tool call against the approval rules:
    ///
    /// 1. If [`auto_approve`] is `true`, the call is immediately [`Approved`].
    /// 2. If the tool name is in the [`allow_list`], the call is immediately
    ///    [`Approved`].
    /// 3. Otherwise, the user is prompted on stderr with the tool name and
    ///    description, and asked to choose:
    ///    - `y` (yes): Approve this single call.
    ///    - `n` (no): Deny this single call.
    ///    - `a` (always): Approve this call and add the tool to the allow list
    ///      for future calls.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The name of the tool being called (e.g., `"read_file"`).
    /// * `description` - A human-readable description of what the tool call does.
    ///
    /// # Returns
    ///
    /// An [`ApprovalDecision`] indicating the outcome of the check.
    ///
    /// [`auto_approve`]: Self::auto_approve
    /// [`allow_list`]: Self::allow_list
    /// [`Approved`]: ApprovalDecision::Approved
    pub fn check(&mut self, tool_name: &str, description: &str) -> ApprovalDecision {
        if self.auto_approve || self.allow_list.contains(tool_name) {
            return ApprovalDecision::Approved;
        }

        // Prompt user for approval
        eprintln!(
            "Tool call: {}\nDescription: {}\n[y]es / [n]o / [a]lways: ",
            tool_name, description
        );

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => ApprovalDecision::Approved,
            "a" | "always" => {
                self.allow_list.insert(tool_name.to_string());
                ApprovalDecision::AlwaysApproved
            }
            _ => ApprovalDecision::Denied,
        }
    }
}

/// Represents the outcome of a tool approval check.
///
/// This enum is returned by [`ToolApproval::check`] to indicate what action
/// should be taken for a tool call.
///
/// # Variants
///
/// * [`Approved`] - The tool call is allowed to proceed. This can happen when:
///   - Global auto-approve is enabled.
///   - The tool is in the allow list.
///   - The user explicitly approved the call.
/// * [`Denied`] - The tool call is rejected. This happens when the user
///   explicitly denies the call.
/// * [`AlwaysApproved`] - The tool call is approved and the tool has been
///   added to the allow list for future automatic approval.
///
/// [`Approved`]: ApprovalDecision::Approved
/// [`Denied`]: ApprovalDecision::Denied
/// [`AlwaysApproved`]: ApprovalDecision::AlwaysApproved
pub enum ApprovalDecision {
    /// The tool call is approved and can proceed.
    Approved,
    /// The tool call is denied and should not proceed.
    Denied,
    /// The tool call is approved, and the tool has been added to the allow list
    /// so future calls of the same tool will be automatically approved.
    AlwaysApproved,
}
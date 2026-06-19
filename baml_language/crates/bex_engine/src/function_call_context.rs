use indexmap::IndexMap;
use sys_types::{CallId, CancellationToken};

/// Per-call context passed to [`crate::BexEngine::call_function`].
///
/// Constructed via [`FunctionCallContextBuilder`].
pub struct FunctionCallContext {
    pub call_id: CallId,
    pub cancel: CancellationToken,
    /// Named `TypeVar` bindings for a generic call. Each entry is
    /// `TypeVar name -> concrete type`; insertion order is the callee's De
    /// Bruijn order. Sourced from a host SDK call (`CallFunctionArgs.type_args`)
    /// or from internal Rust callers invoking generic stdlib functions like
    /// `baml.json.to_string<T>` (which bind their `T` by name here). The engine
    /// lowers these to the positional `type_args` slot by matching names against
    /// the callee's generic params in `set_entry_point_with_type_args`.
    /// Empty for non-generic / internal calls.
    pub type_args: IndexMap<String, baml_type::RuntimeTy>,
}

/// Builder for `FunctionCallContext`.
pub struct FunctionCallContextBuilder {
    call_id: CallId,
    cancel: Option<CancellationToken>,
    type_args: Option<IndexMap<String, baml_type::RuntimeTy>>,
}

impl FunctionCallContextBuilder {
    pub fn new(call_id: CallId) -> Self {
        Self {
            call_id,
            cancel: None,
            type_args: None,
        }
    }

    #[must_use]
    pub fn build(self) -> FunctionCallContext {
        FunctionCallContext {
            call_id: self.call_id,
            cancel: self.cancel.unwrap_or_default(),
            type_args: self.type_args.unwrap_or_default(),
        }
    }

    /// Seed named `TypeVar` bindings for a generic call. Insertion order should
    /// be the callee's De Bruijn order. The engine resolves them to positional
    /// slots against the callee's generic params.
    #[must_use]
    pub fn with_type_args(
        mut self,
        type_args: IndexMap<String, baml_type::RuntimeTy>,
    ) -> Self {
        self.type_args = Some(type_args);
        self
    }

    #[must_use]
    pub fn with_cancel_token(mut self, cancel: CancellationToken) -> Self {
        self.cancel = Some(cancel);
        self
    }
}

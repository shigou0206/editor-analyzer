use crate::core::types::*;
use std::collections::HashMap;
use serde_json;
use uuid;
use futures::stream::BoxStream;
use std::future::Future;

#[derive(Debug, Clone)]
pub struct AiContext {
    pub source_code: SourceCode,
    pub symbols: Vec<Symbol>,
    pub diagnostics: Vec<Diagnostic>,
    pub file_context: FileContext,
    pub trace_id: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl AiContext {
    pub fn new(source_code: SourceCode, file_context: FileContext) -> Self {
        Self {
            source_code,
            symbols: Vec::new(),
            diagnostics: Vec::new(),
            file_context,
            trace_id: uuid::Uuid::new_v4().to_string(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_symbols(mut self, symbols: Vec<Symbol>) -> Self {
        self.symbols = symbols;
        self
    }

    pub fn with_diagnostics(mut self, diagnostics: Vec<Diagnostic>) -> Self {
        self.diagnostics = diagnostics;
        self
    }

    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }
}

#[derive(Debug, Clone)]
pub enum AiRequestType {
    CodeGeneration { prompt: String },
    CodeExplanation { target_span: Option<Span> },
    CodeFix { diagnostic: Diagnostic },
    CodeRefactor { target_span: Span, instruction: String },
    CommentGeneration { target_span: Span },
}

#[derive(Debug, Clone)]
pub struct AiRequest {
    pub request_type: AiRequestType,
    pub context: AiContext,
    pub options: HashMap<String, serde_json::Value>,
}

impl AiRequest {
    pub fn new(request_type: AiRequestType, context: AiContext) -> Self {
        Self {
            request_type,
            context,
            options: HashMap::new(),
        }
    }

    pub fn with_option(mut self, key: String, value: serde_json::Value) -> Self {
        self.options.insert(key, value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct AiResponse {
    pub content: String,
    pub trace_id: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub edits: Option<Vec<TextEdit>>,
}

impl AiResponse {
    pub fn new(content: String, trace_id: String) -> Self {
        Self {
            content,
            trace_id,
            metadata: HashMap::new(),
            edits: None,
        }
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_edits(mut self, edits: Vec<TextEdit>) -> Self {
        self.edits = Some(edits);
        self
    }
}

pub trait AiProvider {
    type Request;
    type Response;
    type Error;
    
    fn generate_code(&self, request: Self::Request) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;
    fn explain_code(&self, code: &str, context: &AiContext) -> impl Future<Output = Result<String, Self::Error>> + Send;
    fn suggest_improvements(&self, code: &str, context: &AiContext) -> impl Future<Output = Result<Vec<String>, Self::Error>> + Send;
    fn stream_response(&self, request: Self::Request) -> impl Future<Output = Result<BoxStream<'static, Result<Self::Response, Self::Error>>, Self::Error>> + Send;
} 
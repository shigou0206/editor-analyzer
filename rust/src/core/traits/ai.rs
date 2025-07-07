use crate::core::types::*;
use std::collections::HashMap;
use serde_json;
use uuid;
use futures::future::BoxFuture;


/// 通用 AI 请求 trait
pub trait AiRequest: Send + Sync {
    type Context;
    type Options;
    
    fn context(&self) -> &Self::Context;
    fn options(&self) -> &Self::Options;
    fn request_type(&self) -> &str;
}

/// 通用 AI 响应 trait
pub trait AiResponse: Send + Sync {
    type Content;
    type Metadata;
    
    fn content(&self) -> &Self::Content;
    fn metadata(&self) -> &Self::Metadata;
    fn trace_id(&self) -> &str;
}

/// AI 上下文接口
pub trait AiContext: Send + Sync {
    fn source_code(&self) -> &str;
    fn language(&self) -> &str;
    fn file_id(&self) -> &str;
    fn trace_id(&self) -> &str;
    fn metadata(&self) -> &HashMap<String, serde_json::Value>;
}

/// AI 选项接口
pub trait AiOptions: Send + Sync {
    fn get(&self, key: &str) -> Option<&serde_json::Value>;
    fn temperature(&self) -> f32;
    fn max_tokens(&self) -> Option<usize>;
}

/// 具体的 AI 上下文实现
#[derive(Debug, Clone)]
pub struct ConcreteAiContext {
    pub source_code: SourceCode,
    pub symbols: Vec<Symbol>,
    pub diagnostics: Vec<Diagnostic>,
    pub file_context: FileContext,
    pub trace_id: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ConcreteAiContext {
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

impl AiContext for ConcreteAiContext {
    fn source_code(&self) -> &str {
        &self.source_code.content
    }
    
    fn language(&self) -> &str {
        match self.source_code.language {
            Language::Rust => "rust",
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            _ => "unknown",
        }
    }
    
    fn file_id(&self) -> &str {
        &self.file_context.file_id.0
    }
    
    fn trace_id(&self) -> &str {
        &self.trace_id
    }
    
    fn metadata(&self) -> &HashMap<String, serde_json::Value> {
        &self.metadata
    }
}

/// 具体的 AI 选项实现
#[derive(Debug, Clone)]
pub struct ConcreteAiOptions {
    pub options: HashMap<String, serde_json::Value>,
    pub temperature: f32,
    pub max_tokens: Option<usize>,
}

impl ConcreteAiOptions {
    pub fn new() -> Self {
        Self {
            options: HashMap::new(),
            temperature: 0.7,
            max_tokens: None,
        }
    }
    
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }
    
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    
    pub fn with_option(mut self, key: String, value: serde_json::Value) -> Self {
        self.options.insert(key, value);
        self
    }
}

impl AiOptions for ConcreteAiOptions {
    fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.options.get(key)
    }
    
    fn temperature(&self) -> f32 {
        self.temperature
    }
    
    fn max_tokens(&self) -> Option<usize> {
        self.max_tokens
    }
}

/// 具体的 AI 请求实现
#[derive(Debug, Clone)]
pub struct ConcreteAiRequest {
    pub request_type: String,
    pub context: ConcreteAiContext,
    pub options: ConcreteAiOptions,
}

impl ConcreteAiRequest {
    pub fn new(request_type: String, context: ConcreteAiContext) -> Self {
        Self {
            request_type,
            context,
            options: ConcreteAiOptions::new(),
        }
    }
    
    pub fn with_options(mut self, options: ConcreteAiOptions) -> Self {
        self.options = options;
        self
    }
}

impl AiRequest for ConcreteAiRequest {
    type Context = ConcreteAiContext;
    type Options = ConcreteAiOptions;
    
    fn context(&self) -> &Self::Context {
        &self.context
    }
    
    fn options(&self) -> &Self::Options {
        &self.options
    }
    
    fn request_type(&self) -> &str {
        &self.request_type
    }
}

/// 具体的 AI 响应实现
#[derive(Debug, Clone)]
pub struct ConcreteAiResponse {
    pub content: String,
    pub trace_id: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub edits: Option<Vec<TextEdit>>,
}

impl ConcreteAiResponse {
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

impl AiResponse for ConcreteAiResponse {
    type Content = String;
    type Metadata = HashMap<String, serde_json::Value>;
    
    fn content(&self) -> &Self::Content {
        &self.content
    }
    
    fn metadata(&self) -> &Self::Metadata {
        &self.metadata
    }
    
    fn trace_id(&self) -> &str {
        &self.trace_id
    }
}

/// AI 服务提供者 trait - 使用泛型解耦
pub trait AiProvider<Req, Resp>: Send + Sync 
where
    Req: AiRequest,
    Resp: AiResponse,
{
    type Error: std::error::Error + Send + Sync + 'static;
    type StreamResponse;
    
    fn generate_code(&self, request: Req) -> BoxFuture<'_, Result<Resp, Self::Error>>;
    fn explain_code(&self, code: &str, context: &dyn AiContext) -> BoxFuture<'_, Result<String, Self::Error>>;
    fn suggest_improvements(&self, code: &str, context: &dyn AiContext) -> BoxFuture<'_, Result<Vec<String>, Self::Error>>;
    fn stream_response(&self, request: Req) -> BoxFuture<'_, Result<Self::StreamResponse, Self::Error>>;
    
    fn capabilities(&self) -> AiCapabilities;
    fn is_available(&self) -> bool;
    fn config(&self) -> AiConfig;
}

/// AI 服务能力
#[derive(Debug, Clone, PartialEq)]
pub struct AiCapabilities {
    pub supports_completion: bool,
    pub supports_explanation: bool,
    pub supports_refactoring: bool,
    pub supports_documentation: bool,
    pub supports_test_generation: bool,
    pub supports_complexity_analysis: bool,
    pub supports_smell_detection: bool,
    pub supports_optimization: bool,
    pub supports_code_generation: bool,
    pub supports_streaming: bool,
    pub max_tokens: Option<usize>,
    pub supported_languages: Vec<String>,
}

impl Default for AiCapabilities {
    fn default() -> Self {
        Self {
            supports_completion: true,
            supports_explanation: true,
            supports_refactoring: true,
            supports_documentation: true,
            supports_test_generation: true,
            supports_complexity_analysis: true,
            supports_smell_detection: true,
            supports_optimization: true,
            supports_code_generation: true,
            supports_streaming: false,
            max_tokens: None,
            supported_languages: vec!["rust".to_string(), "python".to_string(), "javascript".to_string()],
        }
    }
}

/// AI 服务配置
#[derive(Debug, Clone)]
pub struct AiConfig {
    pub api_key: Option<String>,
    pub endpoint: String,
    pub timeout: std::time::Duration,
    pub max_retries: usize,
    pub temperature: f32,
    pub max_tokens: usize,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            endpoint: "https://api.openai.com/v1".to_string(),
            timeout: std::time::Duration::from_secs(30),
            max_retries: 3,
            temperature: 0.7,
            max_tokens: 2048,
        }
    }
}

 
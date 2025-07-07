// 错误 code 常量分层定义
pub mod parser {
    pub const SYNTAX_ERROR: &str = "parser.syntax_error";
    pub const UNSUPPORTED_LANGUAGE: &str = "parser.unsupported_language";
    pub const ALL: &str = "parse_error";
}
pub mod semantic {
    pub const SYMBOL_NOT_FOUND: &str = "semantic.symbol_not_found";
    pub const TYPE_MISMATCH: &str = "semantic.type_mismatch";
    pub const ALL: &str = "semantic_error";
}
pub mod ai {
    pub const API_CALL_FAILED: &str = "ai.api_call_failed";
    pub const INVALID_RESPONSE: &str = "ai.invalid_response";
    pub const ALL: &str = "ai_error";
}
pub mod lsp {
    pub const CONNECTION_FAILED: &str = "lsp.connection_failed";
    pub const INVALID_REQUEST: &str = "lsp.invalid_request";
    pub const ALL: &str = "lsp_error";
}
pub mod file {
    pub const FILE_NOT_FOUND: &str = "file.file_not_found";
    pub const PERMISSION_DENIED: &str = "file.permission_denied";
    pub const ALL: &str = "file_error";
}
pub mod config {
    pub const CONFIG_NOT_FOUND: &str = "config.config_not_found";
    pub const INVALID_FORMAT: &str = "config.invalid_format";
    pub const ALL: &str = "config_error";
}
pub mod network {
    pub const TIMEOUT: &str = "network.timeout";
    pub const ALL: &str = "network_error";
}
pub mod internal {
    pub const PANIC: &str = "internal.panic";
    pub const JSON_ERROR: &str = "internal.json_error";
    pub const ALL: &str = "internal_error";
}
pub mod io {
    pub const IO_ERROR: &str = "io.io_error";
    pub const ALL: &str = "io_error";
}
pub mod reqwest {
    pub const REQWEST_ERROR: &str = "reqwest.reqwest_error";
    pub const ALL: &str = "reqwest_error";
}
pub const CONFIG_KEY_NOT_FOUND: &str = "config_key_not_found";
pub const CONFIG_DESERIALIZE_ERROR: &str = "config_deserialize_error";
pub const CONFIG_SERIALIZE_ERROR: &str = "config_serialize_error";
// ...如有其它 code，后续补充 
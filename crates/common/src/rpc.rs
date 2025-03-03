use std::fmt::Display;

use rmp_serde::{from_slice, to_vec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcRequest {
    pub method: String,
    pub params: Vec<u8>, // Raw MessagePack bytes for params
    pub id: Option<u64>,
}

impl RpcRequest {
    /// Creates a new RPC request.
    pub fn new(method: String, params: Vec<u8>, id: Option<u64>) -> Self {
        Self { method, params, id }
    }

    /// Deserializes parameters into the specified type.
    pub fn parse_params<T: for<'a> Deserialize<'a>>(&self) -> Result<T, RpcError> {
        from_slice(&self.params).map_err(|e| {
            RpcError::new(
                RpcErrorCode::InvalidParams,
                format!("Failed to parse params: {}", e),
            )
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcResponse {
    pub result: Option<Vec<u8>>,
    pub error: Option<RpcError>,
    pub id: Option<u64>,
}

impl RpcResponse {
    pub fn success_unchecked<T: Serialize>(result: T, id: Option<u64>) -> Self {
        let result_bytes = to_vec(&result).ok();
        RpcResponse {
            result: result_bytes,
            error: None,
            id,
        }
    }

    pub fn error(error: RpcError, id: Option<u64>) -> Self {
        RpcResponse {
            result: None,
            error: Some(error),
            id,
        }
    }

    pub fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        rmp_serde::to_vec(self).map_err(Into::into)
    }

    /// Deserializes the result into the specified type.
    pub fn parse_result<T: for<'a> Deserialize<'a>>(&self) -> Result<T, RpcError> {
        match &self.result {
            Some(bytes) => from_slice(bytes).map_err(|e| {
                RpcError::new(
                    RpcErrorCode::InternalError,
                    format!("Failed to parse result: {}", e),
                )
            }),
            None => Err(self.error.clone().unwrap_or_else(|| {
                RpcError::new(
                    RpcErrorCode::InternalError,
                    "Response contains no result or error".to_string(),
                )
            })),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Vec<u8>>,
}

impl Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl RpcError {
    /// Creates a new RPC error.
    pub fn new(code: RpcErrorCode, message: String) -> Self {
        Self {
            code: code.code(),
            message,
            data: None,
        }
    }

    /// Creates a new RPC error with serialized data.
    pub fn new_with_data<T: Serialize>(
        code: RpcErrorCode,
        message: String,
        data: T,
    ) -> Result<Self, anyhow::Error> {
        let data_bytes = to_vec(&data)?;
        Ok(Self {
            code: code.code(),
            message,
            data: Some(data_bytes),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RpcErrorCode {
    // JSON RPC 2.0 standard error codes
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
    // Server error range (-32000 to -32099)
    ServerError = -32000, // Base server error code
}

impl RpcErrorCode {
    pub fn code(&self) -> i32 {
        *self as i32
    }

    pub fn description(&self) -> &'static str {
        match self {
            RpcErrorCode::ParseError => "Parse error",
            RpcErrorCode::InvalidRequest => "Invalid request",
            RpcErrorCode::MethodNotFound => "Method not found",
            RpcErrorCode::InvalidParams => "Invalid params",
            RpcErrorCode::InternalError => "Internal error",
            RpcErrorCode::ServerError => "Server error",
        }
    }
}

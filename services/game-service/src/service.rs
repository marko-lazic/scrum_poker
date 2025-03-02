use common::prelude::{RpcError, RpcRequest, RpcResponse};

use crate::resources::Resources;

// A service context containing resources
pub struct ServiceContext {
    pub resources: &'static Resources,
    pub request: RpcRequest,
}

// Trait for synchronous services
pub trait Service: Send + Sync + 'static {
    fn call(&self, ctx: ServiceContext) -> Result<RpcResponse, RpcError>;
}

// Implement Service for function pointers
impl<F> Service for F
where
    F: Fn(ServiceContext) -> Result<RpcResponse, RpcError> + Send + Sync + 'static,
{
    fn call(&self, ctx: ServiceContext) -> Result<RpcResponse, RpcError> {
        self(ctx)
    }
}

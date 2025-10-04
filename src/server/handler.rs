use std::{marker::PhantomData, pin::Pin, sync::Arc};

use crate::{Request, Response};

pub trait Handler<S>: Send + Sync + 'static {
    fn call(
        &self,
        request: Request,
        response: Response,
        state: Option<Arc<S>>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

pub struct AsyncHandler<F, Fut, S>
where
    F: Fn(Request, Response, Arc<S>) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    pub handler: F,
    _phantom: PhantomData<fn() -> (Fut, S)>,
}

impl<F, Fut, S> Handler<S> for AsyncHandler<F, Fut, S>
where
    F: Fn(Request, Response, Arc<S>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
    S: Send + Sync + 'static,
{
    fn call(
        &self,
        req: Request,
        res: Response,
        state: Option<Arc<S>>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let state = state.expect("Server state not set!");
        let fut = (self.handler)(req, res, state);
        Box::pin(fut)
    }
}

impl<F, Fut, S> AsyncHandler<F, Fut, S>
where
    F: Fn(Request, Response, Arc<S>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
    S: Send + Sync + 'static,
{
    pub fn new(handler: F) -> Self {
        Self {
            handler,
            _phantom: PhantomData,
        }
    }
}

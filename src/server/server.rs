use crate::{
    Request,
    server::{
        handler::{AsyncHandler, Handler},
        http::{HttpMethod, HttpStatus},
        request::get_request_info,
        response::Response,
    },
};
use std::{collections::HashMap, sync::Arc};
use tokio::net::{TcpListener, TcpStream};

type Routes<S> = HashMap<(HttpMethod, String), Arc<dyn Handler<S>>>;

pub struct HttpServer<S = ()> {
    routes: Routes<S>,
    state: Option<Arc<S>>,
}

impl<S> HttpServer<S>
where
    S: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            state: None,
        }
    }

    pub fn with_state(mut self, state: S) -> Self {
        self.state = Some(Arc::new(state));
        self
    }

    // ------------------------------------------------------------
    // Routes start
    // ------------------------------------------------------------
    pub fn delete<F, Fut>(&mut self, path: String, handler: F)
    where
        F: Fn(Request, Response, Arc<S>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h = AsyncHandler::new(handler);
        self.routes.insert((HttpMethod::Delete, path), Arc::new(h));
    }

    pub fn get<F, Fut>(&mut self, path: String, handler: F)
    where
        F: Fn(Request, Response, Arc<S>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let h = AsyncHandler::new(handler);

        self.routes.insert((HttpMethod::Get, path), Arc::new(h));
    }

    pub fn patch<F, Fut>(&mut self, path: String, handler: F)
    where
        F: Fn(Request, Response, Arc<S>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h = AsyncHandler::new(handler);

        self.routes.insert((HttpMethod::Patch, path), Arc::new(h));
    }

    pub fn post<F, Fut>(&mut self, path: String, handler: F)
    where
        F: Fn(Request, Response, Arc<S>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        let h = AsyncHandler::new(handler);

        self.routes.insert((HttpMethod::Post, path), Arc::new(h));
    }

    // ------------------------------------------------------------
    // Routes end
    // ------------------------------------------------------------

    async fn handle_connection(routes: Routes<S>, mut stream: TcpStream, state: Option<Arc<S>>) {
        let request_info = get_request_info(&mut stream).await;

        let handler = routes.get(&(
            request_info.method.clone(),
            String::from(&request_info.path),
        ));

        let mut response: Response = Response::new(stream);

        if let Some(handler) = handler {
            handler.call(request_info, response, state).await;
        } else {
            response
                .send(HttpStatus::NotFound, "Route not found.", "text/plain")
                .await;
        }
    }

    pub async fn listen(&mut self, port: u16) -> std::io::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:".to_string() + &port.to_string()).await?;

        let routes = self.routes.clone();
        let state = self.state.clone();

        loop {
            let (stream, _) = listener.accept().await?;

            let routes = routes.clone();
            let state = state.clone();

            tokio::spawn(async {
                HttpServer::handle_connection(routes, stream, state).await;
            });
        }
    }
}

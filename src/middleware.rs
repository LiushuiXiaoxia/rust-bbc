use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpRequest,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::task::{Context, Poll};

pub struct RequestHandler;

impl RequestHandler {
    pub fn new() -> Self {
        RequestHandler
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestHandlerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestHandlerMiddleware { service }))
    }
}

pub struct RequestHandlerMiddleware<S> {
    service: S,
}

async fn handle_get(req: &HttpRequest, path: &str) -> Result<(), Error> {
    println!("处理 GET 请求: {}, 完整请求: {:?}", path, req);
    Ok(())
}

async fn handle_put(req: &HttpRequest, path: &str) -> Result<(), Error> {
    println!("处理 PUT 请求: {}, 完整请求: {:?}", path, req);
    Ok(())
}

async fn handle_delete(req: &HttpRequest, path: &str) -> Result<(), Error> {
    println!("处理 DELETE 请求: {}, 完整请求: {:?}", path, req);
    Ok(())
}

async fn handle_head(req: &HttpRequest, path: &str) -> Result<(), Error> {
    println!("处理 HEAD 请求: {}, 完整请求: {:?}", path, req);
    Ok(())
}

impl<S, B> Service<ServiceRequest> for RequestHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        let fut = self.service.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(res) => {
                    let req = res.request();
                    match *req.method() {
                        actix_web::http::Method::GET => handle_get(req, &path).await?,
                        actix_web::http::Method::PUT => handle_put(req, &path).await?,
                        actix_web::http::Method::DELETE => handle_delete(req, &path).await?,
                        actix_web::http::Method::HEAD => handle_head(req, &path).await?,
                        _ => (),
                    }
                    Ok(res)
                }
                Err(e) => Err(e),
            }
        })
    }
}

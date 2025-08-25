use color_eyre::eyre::Result;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use log::{debug, error};
use std::net::SocketAddr;
use tiefbloglib::path_resolution::{self, PathResolver};
use tiefbloglib::render::render_page;
use tokio::net::TcpListener;

pub(crate) async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    debug!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                error!(
                    "An unrecoverable errror occurred in the main listener:\n{}",
                    err
                );
            }
        });
    }
}

async fn hello(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>> {
    let path_resolver = get_path_resolver(req.method());

    Ok(Response::new(Full::from(Bytes::from(render_page(
        &path_resolver(req.uri().path())(req.uri().query())?,
    )?))))
}

pub fn get_path_resolver(method: &Method) -> PathResolver {
    // We possibly will have to add more methods in the future
    // but right now, let's just do get.
    match method {
        &Method::GET => path_resolution::resolve_get_path,
        _ => path_resolution::resolve_unknown_path,
    }
}

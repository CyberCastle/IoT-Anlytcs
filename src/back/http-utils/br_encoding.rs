use actix_service::{Service, Transform};
use actix_web::{
    dev::ServiceRequest,
    dev::ServiceResponse,
    http::header::{HeaderValue, CONTENT_ENCODING},
    Error,
};

use futures::{
    future::{ok, Ready},
    Future,
};

use std::{
    ffi::OsStr,
    task::{Context, Poll},
};

use std::{path::Path, pin::Pin};

/**
 * Este midleware permite añadir el header "Content-Encoding: br" a todos los archivos estáticos servidos (usando actix_files::Files),
 * cuya extensión está definida en el arreglo "DEFAULT_EXTENSIONS_COMPRESSED". Cómo prerequisito, todos los archivos ya deben estar
 * comprimidos, usando el algoritmo brotli
 */

pub const DEFAULT_EXTENSIONS_COMPRESSED: [&str; 9] = [
    "js", "css", "html", "svg", "eot", "woff2", "woff", "ttf", "/",
];
pub struct BrotliEncodingHeader;

impl<S, B> Transform<S, ServiceRequest> for BrotliEncodingHeader
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = BrotliEncodingHeaderMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(BrotliEncodingHeaderMiddleware { service })
    }
}

pub struct BrotliEncodingHeaderMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for BrotliEncodingHeaderMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Obtenemos la referencia de la petición HTTTP.
        let req_ref = &req;

        // A partir de la referencia, obtenemos la ruta de la petición HTTP,
        // que hace referencia a un archivo estático a servir.
        let file_extension = Path::new(req_ref.path())
            .extension() // Obtenemos la extensión del archivo (js, css, html, etc.)
            .and_then(OsStr::to_str) // La extensión se obtiene como string
            .unwrap_or("/"); // Si no viene la extensión, devolvemos un string vacío
        let extension_finded = DEFAULT_EXTENSIONS_COMPRESSED.contains(&file_extension);

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            // Si la extensión está presente en el arreglo, asumimos que el archivo está comprimido con el algoritmo Brotli,
            // de tal forma que, por medio del header "Content-Encoding", le informamos al browser que el archivo transmitido
            // está comprimido.
            if extension_finded {
                let headers = res.headers_mut();
                headers.append(CONTENT_ENCODING, HeaderValue::from_static("br"));
            }

            return Ok(res);
        })
    }
}

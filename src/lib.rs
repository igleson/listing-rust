use worker::*;

#[event(fetch)]
async fn fetch(_req: HttpRequest, _env: Env, _ctx: Context) -> Result<HttpResponse> {
    let router = Router::new();

    router
        .get_async("/test/:name", |_r, _c| async {
            if let Some(name) = ctx.param("name") {
                return Response::ok(format!("hello {}", name));
            };
            Response::error("Bad Request", 400)
        })
        .get_async("another", |_r, _c| async { Response::ok("hello another") })
        .run(req, env)
        .await;

    console_error_panic_hook::set_once();
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Body::empty())?)
}

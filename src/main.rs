use web_spooder::*;

fn api(request: &mut request::Request) -> response::Response {
    fn root(_request: &mut request::Request) -> response::Response {
        return response::Response {
            status: response::status(200),
            body: response::Body {
                content_type: "HTML\n".to_string(),
                content: "<p>Welcome to the a!</p>".to_string(),
            },
        }
    }

    fn b(_request: &mut request::Request) -> response::Response {
        return response::Response {
            status: response::status(200),
            body: response::Body {
                content_type: "HTML\n".to_string(),
                content: "<p>Welcome to the c!</p>".to_string(),
            },
        }
    }

    fn c(_request: &mut request::Request) -> response::Response {
        return response::Response {
            status: response::status(200),
            body: response::Body {
                content_type: "HTML\n".to_string(),
                content: "<p>Welcome to the b!</p>".to_string(),
            },
        }
    }

    let mut router: router::Router = router::Router::new();
    router.add("b".to_string(), b);
    router.add("c".to_string(), c);
    router.add("".to_string(), root);

    return router.respond(request)

}

fn main() {
    let mut router: router::Router = router::Router::new();
    router.add("a".to_string(), api);

    let app: app::App = app::App::new(router);

    app.run();
}
#[cfg(feature = "ssr")]
mod ssr {
    use axum::{
        body::Body as AxumBody,
        extract::{FromRef, State},
        http::Request,
        response::{IntoResponse, Response},
    };

    use leptos::{config::LeptosOptions, prelude::*};
    use leptos_axum::{handle_server_fns_with_context, AxumRouteListing};
    use leptos_router::RouteListing;
    use sqlx::SqlitePool;
    use takenote::app::shell;

    /// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
    /// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub pool: SqlitePool,
        pub routes: Vec<AxumRouteListing>,
    }

    pub async fn server_fn_handler(
        State(app_state): State<AppState>,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        handle_server_fns_with_context(
            move || {
                provide_context(app_state.pool.clone());
            },
            request,
        )
        .await
    }

    pub async fn leptos_routes_handler(
        State(app_state): State<AppState>,
        req: Request<AxumBody>,
    ) -> Response {
        let handler = leptos_axum::render_app_to_stream_with_context(
            move || {
                provide_context(app_state.pool.clone());
            },
            {
                let leptos_options = app_state.leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        );
        handler(req).await.into_response()
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::get;
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::sqlite::SqlitePoolOptions;
    use takenote::app::*;

    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let pool = SqlitePoolOptions::new()
        .connect("sqlite:notation.db?mode=rwc")
        .await
        .expect("Could not make pool.");

    if let Err(e) = sqlx::migrate!().run(&pool).await {
        eprintln!("{e:?}");
    }

    let site_root = leptos_options.site_root.to_string();
    let serve_dir =
        tower_http::services::ServeDir::new(&site_root).append_index_html_on_directories(false);

    let app_state = ssr::AppState {
        leptos_options,
        pool,
        routes: routes.clone(),
    };

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(ssr::server_fn_handler).post(ssr::server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(ssr::leptos_routes_handler))
        .fallback_service(serve_dir)
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}

use std::str::FromStr;

use leptos::{either::Either, prelude::*};
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

#[server(FetchPages)]
async fn fetch_page(page_id: Uuid) -> Result<crate::data::PageBlock, ServerFnError<String>> {
    use crate::data::{build_page_tree, PageBlock};
    tracing::info!("Fetching page...");
    let blocks = crate::data::block::SqlBlock::list(page_id)
        .await
        .map_err(|ae| ServerFnError::WrappedServerError(ae.to_string()))?;
    let page = build_page_tree(page_id, &blocks)
        .map_err(|err| ServerFnError::WrappedServerError(err.to_string()))?;
    tracing::info!(page = ?page);
    Ok(page)
}

#[component]
pub fn EmptyPage() -> impl IntoView {
    view! {
        <div>
        "Nothing to see here."
        </div>
    }
}

#[component]
pub fn EditorPage() -> impl IntoView {
    let params = use_params_map();
    let word = move || params.with(|params| params.get("page_id").clone());

    let results = Resource::new(
        move || word().unwrap(),
        move |query| async move { fetch_page(Uuid::from_str(query.as_str()).unwrap()).await },
    );

    view! {
        <div>
        <Transition>
        {move || Suspend::new(async move {
            let result = results.await;
            match result {
                Ok(page) => Either::Left(view! {
                    <div>"Got a page: "{page.props.title}</div>
                }),
                Err(err) => Either::Right(view! { <div>{err.to_string()}</div>})
            }
        })}
        </Transition>
        </div>
    }
}

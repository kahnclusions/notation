use std::str::FromStr;

use crate::{data::Block, ui::anchor::A};
use leptos::{either::Either, prelude::*};
use uuid::Uuid;

use crate::data::PageBlock;

#[server(FetchPages)]
async fn fetch_pages() -> Result<Vec<crate::data::PageBlock>, ServerFnError<String>> {
    use crate::data::{build_page_tree, PageBlock};
    tracing::info!("Fetching...");
    let pages = crate::data::block::SqlBlock::list_pages()
        .await
        .map_err(|ae| ServerFnError::WrappedServerError(ae.to_string()))?;
    let parents: Vec<PageBlock> = pages
        .iter()
        .filter_map(|page| {
            if page.parent_id.is_none() {
                let page_id = Uuid::from_str(page.id.as_str()).unwrap();
                build_page_tree(page_id, &pages).ok()
            } else {
                None
            }
        })
        .collect();
    tracing::info!(pages = ?parents);
    Ok(parents)
}

#[component]
pub fn PageNav() -> impl IntoView {
    let pages = Resource::new(|| (), move |_| fetch_pages());

    view! {
        <Transition>
        {move || Suspend::new(async move {
            match pages.await {
                Ok(pages) => Either::Left(view! {
                    <ul>{pages.into_iter().map(|p| view! {
                        <PageList page=p />
                    }).collect::<Vec<_>>()}</ul>
                }),
                Err(e) => Either::Right(view! { <div>"Error: "{e.to_string()}</div>})
            }
        })}
        </Transition>
    }
}

#[component]
pub fn PageList(page: PageBlock) -> impl IntoView {
    view! {
        <li>
            <A href={format!("/page/{}", page.id)}>{page.props.title}</A>
            <ul>
            {page.children.into_iter().filter_map(|p|
                match p {
                    Block::Page(child_p) => {
                        Some(view! {
                            <PageList page=child_p />
                        }.into_any())
                    },
                    _ => None
                }).collect_view()}
            </ul>
        </li>
    }
}

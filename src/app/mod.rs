mod editor;
mod page_nav;

use editor::{EditorPage, EmptyPage};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};
use page_nav::PageNav;

use crate::ui::{anchor::A, typography::H3};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-white dark:bg-slate-800 text-slate-900 dark:text-white">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/takenote.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <div class="flex flex-row h-full items-stretch justify-between">
        <Router>
            <aside class="p-3 bg-slate-100 dark:bg-slate-900 w-[248px] grow-0 shrink-0">
                <h1 class="font-bold text-2xl font-display">"Notation"</h1>
                <A href="/">"Home"</A>
                <H3>Pages</H3>
    <PageNav />
            </aside>
            <div class="w-[2px] bg-slate-200 dark:bg-slate-950 h-full"></div>
            <main class="p-3 grow">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=EmptyPage/>
                    <Route path=path!("/page/:page_id") view=EditorPage/>
                </Routes>
            </main>
        </Router>
        </div>
    }
}

use leptos::html;
use leptos::prelude::*;
use tailwind_fuse::*;

#[component]
pub fn A(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] href: String,
    #[prop(optional)] node_ref: NodeRef<html::A>,
    children: Children,
) -> impl IntoView {
    view! {
        <a
            class=tw_merge!(
                "font-medium text-foreground underline underline-offset-4 outline-none focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
                class
            )

            href=href
            node_ref=node_ref
        >
            {children()}
        </a>
    }
}

use leptos::html;
use leptos::prelude::*;
use tailwind_fuse::*;

#[component]
pub fn H1(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::H1>,
    children: Children,
) -> impl IntoView {
    view! {
        <h1
            class=tw_merge!(
                "font-roca scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl", class
            )

            node_ref=node_ref
        >
            {children()}
        </h1>
    }
}

#[component]
pub fn H2(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::H2>,
    children: Children,
) -> impl IntoView {
    view! {
        <h2
            class=tw_merge!(
                "mt-10 scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0",
                class
            )

            node_ref=node_ref
        >
            {children()}
        </h2>
    }
}

#[component]
pub fn H3(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::H3>,
    children: Children,
) -> impl IntoView {
    view! {
        <h3
            class=tw_merge!("mt-4 scroll-m-20 text-2xl font-semibold tracking-tight", class)
            node_ref=node_ref
        >
            {children()}
        </h3>
    }
}

#[component]
pub fn H4(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::H4>,
    children: Children,
) -> impl IntoView {
    view! {
        <h4
            class=tw_merge!("mt-4 scroll-m-20 text-xl font-semibold tracking-tight", class)
            node_ref=node_ref
        >
            {children()}
        </h4>
    }
}

#[component]
pub fn Text(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::P>,
    children: Children,
) -> impl IntoView {
    view! {
        <p class=tw_merge!("leading-7", class) node_ref=node_ref>
            {children()}
        </p>
    }
}

#[component]
pub fn TextSpan(
    #[prop(optional, into)] class: String,
    #[prop(optional)] node_ref: NodeRef<html::Span>,
    children: Children,
) -> impl IntoView {
    view! {
        <span class=tw_merge!("leading-7", class) node_ref=node_ref>
            {children()}
        </span>
    }
}

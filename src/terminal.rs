use leptos::prelude::*;

#[component]
pub fn Terminal() -> impl IntoView {
    view! {
        <div class="h-150 border-2 border-line rounded-md bg-linear-to-t from-bg to-surface from-70% w-full">
            <TerminalHeader />
            <div class="mx-5 my-2 flex items-center">
                <span class="text-accent-500">"➜ ~"</span>
                <input type="text" class="mx-2 focus:outline-hidden flex-1" />
                <div class="w-1.5 h-3 bg-accent-500 animate-blink"></div>
            </div>
        </div>
    }
}

#[component]
fn TerminalHeader() -> impl IntoView {
    view! {
        <div class="h-10 border-b border-line bg-surface rounded-t-[inherit] flex items-center gap-2 px-3">
            <Dot class="bg-neutral-700" />
            <Dot class="bg-neutral-700" />
            <Dot class="bg-accent-700" />
            <span class="text-neutral-600 text-xs px-4">"relaforg@dev: ~/portfolio - rsh"</span>
        </div>
    }
}

#[component]
fn Dot(#[prop(into)] class: String) -> impl IntoView {
    view! { <span class=format!("size-3 rounded-full inline-block {class}") aria-hidden="true"></span> }
}

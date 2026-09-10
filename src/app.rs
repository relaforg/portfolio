use crate::{projects::Projects, terminal::Terminal};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-250 px-4">
            <div class="min-h-dvh flex flex-col">
                <Header />
                <div class="flex-1 flex items-center">
                    <Terminal />
                </div>
            </div>
            <Projects />
            <Hr />
            <Footer />
        </div>
    }
}

#[component]
pub fn Hr() -> impl IntoView {
    view! { <hr class="border-line my-1" /> }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="py-6 flex justify-between items-center">
            <p>relaforg<span class="text-accent-500">@</span>dev</p>
            <div class="flex items-center gap-5">
                <nav class="flex items-center gap-5">
                    <a href="#projects">"projects"</a>
                    <a href="#repos">"repos"</a>
                    <a href="#contact">"contact"</a>
                </nav>
                <button class="py-1 px-3 border border-line rounded-sm cursor-pointer">
                    "FR / en"
                </button>
                <button class="cursor-pointer" aria-label="Change theme">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="size-4"
                        aria-hidden="true"
                    >
                        <circle cx="12" cy="12" r="4" />
                        <path d="M12 2v2" />
                        <path d="M12 20v2" />
                        <path d="m4.93 4.93 1.41 1.41" />
                        <path d="m17.66 17.66 1.41 1.41" />
                        <path d="M2 12h2" />
                        <path d="M20 12h2" />
                        <path d="m6.34 17.66-1.41 1.41" />
                        <path d="m19.07 4.93-1.41 1.41" />
                    </svg>
                </button>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let year = js_sys::Date::new_0().get_full_year();
    view! {
        <div class="flex justify-between my-4">
            <p>Rémi Laforgue</p>
            <p>{year}</p>
        </div>
    }
}

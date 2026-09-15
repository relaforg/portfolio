use crate::i18n::*;
use crate::{contact::Contact, projects::Projects, repos::Repos, terminal::Terminal};
use leptos::prelude::*;
use leptos_meta::Html;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18nContextProvider>
            <BgAnim />
            <div class="mx-auto max-w-250 px-4">
                <Header />
                <div class="min-h-dvh flex flex-col snap-start">
                    <div class="flex-1 flex items-center">
                        <Terminal />
                    </div>
                    <div class="flex justify-center">
                        <Chevron />
                    </div>
                </div>
                <div class="snap-start snap-always scroll-mt-25">
                    <Projects />
                    <Repos />
                    <Hr />
                    <Contact />
                    <Hr />
                    <Footer />
                </div>
            </div>
        </I18nContextProvider>
    }
}

#[component]
fn BgAnim() -> impl IntoView {
    view! {
        <div aria-hidden="true" class="pointer-events-none fixed inset-0 -z-1 overflow-hidden">
            <div class="absolute -top-[18vh] -left-[8vw] h-[62vw] w-[62vw] min-h-[420px] min-w-[420px]
            rounded-full opacity-30 blur-[90px] will-change-transform animate-drift-a
            bg-[radial-gradient(circle_at_50%_50%,#80a0ff_0%,transparent_68%)]"></div>
            <div class="absolute -right-[10vw] -bottom-[22vh] h-[55vw] w-[55vw] min-h-[380px] min-w-[380px]
            rounded-full opacity-25 blur-[100px] will-change-transform animate-drift-b
            bg-[radial-gradient(circle_at_50%_50%,#74b2ff_0%,transparent_66%)]"></div>
            <div class="absolute top-[34vh] left-[42vw] h-[44vw] w-[44vw] min-h-[320px] min-w-[320px]
            rounded-full opacity-15 blur-[110px] will-change-transform animate-drift-c
            bg-[radial-gradient(circle_at_50%_50%,#adadf3_0%,transparent_70%)]"></div>
        </div>
    }
}

#[component]
fn Chevron() -> impl IntoView {
    view! {
        <a href="#projects">
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
                class="size-10 mb-10 text-neutral-300"
            >
                <path d="m7 6 5 5 5-5" />
                <path d="m7 13 5 5 5-5" />
            </svg>
        </a>
    }
}

#[component]
pub fn Hr() -> impl IntoView {
    view! { <hr class="border-line my-5" /> }
}

#[component]
fn Sun() -> impl IntoView {
    view! {
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
    }
}

#[component]
fn Moon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="#000000"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="size-4"
        >
            <path d="M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401" />
        </svg>
    }
}

#[component]
fn Header() -> impl IntoView {
    let i18n = use_i18n();

    let (theme, set_theme) = signal(true);

    let toggle_theme = move |_| set_theme.set(!theme.get());
    let get_theme = move || match theme.get() {
        true => "dark",
        false => "light",
    };

    let toggle_locale = move |_| match i18n.get_locale() {
        Locale::en => i18n.set_locale(Locale::fr),
        _ => i18n.set_locale(Locale::en),
    };

    view! {
        <Html attr:data-theme=get_theme attr:class="snap-y snap-mandatory" />
        <header class="p-6 flex justify-between items-center sticky top-1 z-10 isolate">
            <div class="absolute bg-surface/30 backdrop-blur-xl -inset-x-4 -inset-y-0 -z-1 mask-x-from-[calc(100%-1rem)] mask-y-from-[calc(100%-1rem)]"></div>
            <p>relaforg<span class="text-accent-500">@</span>dev</p>
            <div class="flex items-center gap-5">
                <nav class="flex items-center gap-5">
                    <a href="#projects">{t!(i18n, header.projects)}</a>
                    <a href="#contact">"contact"</a>
                </nav>
                <button
                    on:click=toggle_locale
                    class="py-1 px-3 border border-line rounded-sm cursor-pointer"
                >
                    <Show
                        when=move || i18n.get_locale() == Locale::fr
                        fallback=|| view! { "fr / EN" }
                    >
                        "FR / en"
                    </Show>
                </button>
                <button class="cursor-pointer" aria-label="Change theme" on:click=toggle_theme>
                    <Show when=move || theme.get() == true fallback=|| view! { <Moon /> }>
                        <Sun />
                    </Show>
                </button>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let year = js_sys::Date::new_0().get_full_year();
    view! {
        <div class="flex justify-between my-4 pb-4 text-sm text-neutral-500">
            <p>Rémi Laforgue</p>
            <p>{year}</p>
        </div>
    }
}

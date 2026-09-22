use crate::i18n::*;
use crate::{contact::Contact, projects::Projects, repos::Repos, terminal::Terminal};
use leptos::prelude::*;
use leptos_meta::Html;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::hooks::use_location;
use leptos_router::path;
use leptos_use::{use_window_scroll, use_window_size, UseWindowSizeReturn};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Html attr:class="snap-y snap-proximity" />
        <I18nContextProvider>
            <Router>
                <BgAnim />
                <Sidebar />
                <Options />
                <DisplaySlider />
                <div class="mx-auto max-w-250 px-4">
                    <Routes fallback=TerminalView>
                        <Route path=path!("/terminal") view=TerminalView />
                        <Route path=path!("/classic") view=ClassicView />
                    </Routes>
                </div>
            </Router>
        </I18nContextProvider>
    }
}

#[component]
fn DisplaySlider() -> impl IntoView {
    let i18n = use_i18n();
    let pathname = use_location().pathname;
    let is_classic = move || pathname.get() == "/classic";

    view! {
        <div class="fixed top-[18px] right-5 z-[21] flex items-center gap-2.5">
            <span class="text-xs text-neutral-500">{t!(i18n, header.display)}</span>

            <div
                role="group"
                aria-label="Choisir l'affichage"
                class="flex items-center gap-0.5 rounded-lg border border-[var(--color-line)] bg-[var(--color-surface)] p-[3px] text-[13px]"
            >

                <A
                    href="/terminal"
                    attr:class=move || {
                        format!(
                            "rounded-md px-3 py-[5px] transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--color-accent)] {}",
                            if !is_classic() {
                                "bg-accent-700 text-accent-100"
                            } else {
                                "bg-transparent text-neutral-400 hover:text-[var(--color-text)]"
                            },
                        )
                    }
                >
                    {t!(i18n, header.terminal)}
                </A>

                <A
                    href="/classic"
                    attr:class=move || {
                        format!(
                            "rounded-md px-3 py-[5px] transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--color-accent)] {}",
                            if is_classic() {
                                "bg-accent-700 text-accent-100"
                            } else {
                                "bg-transparent text-neutral-400 hover:text-[var(--color-text)]"
                            },
                        )
                    }
                >
                    {t!(i18n, header.classic)}
                </A>
            </div>
        </div>
    }
}

#[component]
fn TerminalView() -> impl IntoView {
    view! {
        <div class="min-h-dvh flex flex-col snap-start">
            <div class="flex-1 flex items-center">
                <Terminal />
            </div>
        </div>
    }
}

#[component]
fn ClassicView() -> impl IntoView {
    view! {
        <div class="mt-15">
            <Projects />
            <Repos />
            <Contact />
            <Hr />
            <Footer />
        </div>
    }
}

#[component]
fn BgAnim() -> impl IntoView {
    view! {
        <div aria-hidden="true" class="pointer-events-none fixed inset-0 -z-1 overflow-hidden">
            <div class="absolute -top-[18vh] -left-[8vw] h-[62vw] w-[62vw] min-h-[420px] min-w-[420px]
            rounded-full opacity-15 blur-[90px] will-change-transform animate-drift-a
            bg-[radial-gradient(circle_at_50%_50%,#80a0ff_0%,transparent_68%)]"></div>
            <div class="absolute -right-[10vw] -bottom-[22vh] h-[55vw] w-[55vw] min-h-[380px] min-w-[380px]
            rounded-full opacity-20 blur-[100px] will-change-transform animate-drift-b
            bg-[radial-gradient(circle_at_50%_50%,#74b2ff_0%,transparent_66%)]"></div>
            <div class="absolute top-[34vh] left-[42vw] h-[44vw] w-[44vw] min-h-[320px] min-w-[320px]
            rounded-full opacity-10 blur-[110px] will-change-transform animate-drift-c
            bg-[radial-gradient(circle_at_50%_50%,#adadf3_0%,transparent_70%)]"></div>
        </div>
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
fn Sidebar() -> impl IntoView {
    let i18n = use_i18n();
    let (_, y) = use_window_scroll();
    let UseWindowSizeReturn { height, .. } = use_window_size();

    let position = move || {
        let h = height.get();
        if h <= 0.0 {
            return 0.0;
        }
        (1.5 * h - y.get()).max(0.5 * h)
    };

    view! {
        <nav
            class="ml-5 fixed -translate-y-1/2 hidden flex-col gap-1 text-neutral-500 xl:flex"
            style=("top", move || format!("{}px", position().to_string()))
        >
            <a href="#projects" class="hover:text-accent-500">
                "— "
                {t!(i18n, header.projects)}
            </a>
            <a href="#repos" class="hover:text-accent-500">
                "— "
                {t!(i18n, header.activity)}
            </a>
            <a href="#contact" class="hover:text-accent-500">
                "— contact"
            </a>
        </nav>
    }
}

#[component]
fn Options() -> impl IntoView {
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
        <Html attr:data-theme=get_theme />
        <div class="p-6 flex justify-between items-center absolute xl:fixed bottom-1 right-1 z-10">
            <div class="flex items-center gap-5">
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
        </div>
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

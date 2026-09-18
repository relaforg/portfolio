use chrono::{DateTime, Local};
use gloo_net::http::Request;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::{t, t_string};
use serde::Deserialize;

use crate::i18n::use_i18n;

#[derive(Deserialize, Clone)]
struct Repo {
    name: String,
    html_url: String,
    description: Option<String>,
    language: Option<String>,
    pushed_at: Option<String>
}

async fn fetch_repos() -> Result<Vec<Repo>, gloo_net::Error> {
    Request::get("https://api.github.com/users/relaforg/repos?sort=pushed&per_page=10")
        .send()
        .await?
        .json()
        .await
}

#[component]
pub fn Repos() -> impl IntoView {
    let i18n = use_i18n();
    let repos = LocalResource::new(fetch_repos);
    view! {
        <section id="repos" class="mb-3 mt-5">
            <h2 class="text-neutral-500">"// " {t!(i18n, activity.title)}</h2>
            <Suspense fallback=move || {
                view! { <p>t!(i18n, activity.loading)</p> }
            }>
                {move || {
                    repos
                        .map(|res| match res {
                            Ok(repos) => {
                                Either::Right(
                                    view! {
                                        <ul class="grid grid-cols-[auto_minmax(0,1fr)_auto]">
                                            {repos
                                                .iter()
                                                .cloned()
                                                .map(|r| view! { <RepoView repo=r /> })
                                                .collect_view()}
                                        </ul>
                                    },
                                )
                            }
                            Err(_e) => Either::Left(view! { <p>{t!(i18n, activity.error)}</p> }),
                        })
                }}
            </Suspense>
            <div class="my-2">
                <a
                    href="https://github.com/relaforg?tab=repositories"
                    target="_blank"
                    class="text-xs text-neutral-500 hover:text-accent-500"
                >
                    {t!(i18n, activity.all_repos)}
                    " ➜"
                </a>
            </div>
        </section>
    }
}

fn format_date(date: Option<String>) -> Signal<String> {
    let Some(date) = date else { return "-".into() };
    let Ok(date) = DateTime::parse_from_rfc3339(&date) else { return "-".into() };

    let i18n = use_i18n();
    let elapsed = Local::now().signed_duration_since(date);
    match elapsed.num_days() {
        0 => Signal::derive(move || t_string!(i18n, date.today).into()),
        1 => Signal::derive(move || t_string!(i18n, date.yesterday).into()),
        days @ 0..=30 => Signal::derive(move || t_string!(i18n, date.days, count = days)),
        days @ 0..=365 => Signal::derive(move || t_string!(i18n, date.months, count = days / 30)),
        days @ _ => Signal::derive(move || t_string!(i18n, date.years, count = days / 365)),
    }
}

#[component]
fn RepoView(repo: Repo) -> impl IntoView {
    view! {
        <li class="p-1 col-span-4 grid grid-cols-subgrid items-baseline gap-x-8 hover:bg-surface text-sm">
            <a
                rel="external noopener noreferrer"
                target="_blank"
                href=repo.html_url
                class="text-accent-500"
            >
                {repo.name}
            </a>
            <p class="truncate">{repo.description}</p>
            <span class="text-neutral-500 justify-self-end">
                {repo.language.unwrap_or_else(|| "-".to_string())}
            </span>
            <span class="text-neutral-500 justify-self-end">{format_date(repo.pushed_at)}</span>
        </li>
        <hr class="col-span-4 h-px border-0 bg-[linear-gradient(to_right,transparent,var(--color-line)_4rem,var(--color-line)_calc(100%-4rem),transparent)]" />
    }
}

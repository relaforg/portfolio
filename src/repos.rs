use gloo_net::http::Request;
use leptos::either::Either;
use leptos::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct Repo {
    name: String,
    html_url: String,
    description: Option<String>,
    pub language: Option<String>,
}

async fn fetch_repos() -> Result<Vec<Repo>, gloo_net::Error> {
    Request::get("shttps://api.github.com/users/relaforg/repos?sort=updated&per_page=20")
        .send()
        .await?
        .json()
        .await
}

#[component]
pub fn Repos() -> impl IntoView {
    let repos = LocalResource::new(fetch_repos);
    view! {
        <h2 id="repos" class="mb-3 text-neutral-500 mt-5">
            "// MON ACTIVITÉ RÉCENTE"
        </h2>
        <Suspense fallback=move || {
            view! { <p>"Chargement..."</p> }
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
                        Err(_e) => {
                            Either::Left(
                                view! { <p>"Impossible de réccuperer l'activité récente."</p> },
                            )
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
fn RepoView(repo: Repo) -> impl IntoView {
    view! {
        <li class="p-1 col-span-3 grid grid-cols-subgrid items-baseline gap-x-8 hover:bg-surface text-sm">
            <a rel="external" href=repo.html_url class="text-accent-500">
                {repo.name}
            </a>
            <p class="truncate">{repo.description}</p>
            <span class="text-neutral-500 justify-self-end">
                {repo.language.unwrap_or_else(|| "-".to_string())}
            </span>
        </li>
        <hr class="col-span-3 h-px border-0 bg-[linear-gradient(to_right,transparent,var(--color-line)_4rem,var(--color-line)_calc(100%-4rem),transparent)]" />
    }
}

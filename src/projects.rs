use std::collections::HashMap;

use leptos::prelude::*;
use leptos_i18n::{t, t_string};

use crate::{app::Hr, i18n::use_i18n};

#[derive(Clone)]
pub struct Project {
    pub name: String,
    pub description: Signal<&'static str>,
    pub techs: Vec<String>,
    pub topics: Vec<Signal<&'static str>>,
    pub github_link: String,
    pub role: Signal<&'static str>,
}

pub fn get_projects() -> HashMap<&'static str, Project> {
    let i18n = use_i18n();

    HashMap::from([
        (
            "TAP",
            Project {
                name: "The Answer Protocol".to_string(),
                role: Signal::derive(move || t_string!(i18n, projects.tap.role)),
                description: Signal::derive(move || t_string!(i18n, projects.tap.description)),
                techs: vec!["Rust".to_string()],
                topics: vec![
                    Signal::derive(move || t_string!(i18n, projects.topics.network)),
                    Signal::derive(move || t_string!(i18n, projects.topics.game)),
                    Signal::derive(move || t_string!(i18n, projects.topics.multiplayer)),
                    Signal::derive(move || "42"),
                ],
                github_link: "https://github.com/Arcanovax/TAP".to_string(),
            },
        ),
        (
            "chip8",
            Project {
                name: "Chip8".to_string(),
                role: Signal::derive(move || t_string!(i18n, projects.chip8.role)),
                description: Signal::derive(move || t_string!(i18n, projects.chip8.description)),
                techs: vec!["Rust".to_string()],
                topics: vec![
                    Signal::derive(move || t_string!(i18n, projects.topics.emulation)),
                    Signal::derive(move || t_string!(i18n, projects.topics.low_level)),
                ],
                github_link: "https://github.com/relaforg/chip8".to_string(),
            },
        ),
        (
            "amazeing",
            Project {
                name: "A Maze Ing".to_string(),
                role: Signal::derive(move || t_string!(i18n, projects.amazeing.role)),
                description: Signal::derive(move || t_string!(i18n, projects.amazeing.description)),
                techs: vec!["Python".to_string()],
                topics: vec![
                    Signal::derive(move || t_string!(i18n, projects.topics.graphic)),
                    Signal::derive(move || "42"),
                ],
                github_link: "https://github.com/relaforg/a_maze_ing".to_string(),
            },
        ),
    ])
}

#[component]
pub fn Projects() -> impl IntoView {
    let i18n = use_i18n();
    let projects = get_projects();
    view! {
        <section id="projects" class="my-3">
            <h2 class="text-neutral-500">"// " {t!(i18n, projects.title)}</h2>
            // <Hr />
            {projects
                .into_iter()
                .map(|n| {
                    view! {
                        <Hr />
                        <ProjectView project=n.1 />
                    }
                })
                .collect_view()}
        </section>
    }
}

#[component]
fn ProjectView(project: Project) -> impl IntoView {
    view! {
        <div class="my-10">
            <div class="flex justify-between items-end my-5">
                <div class="flex sm:items-end sm:gap-2 sm:flex-row flex-col">
                    <h3 class="text-3xl">{project.name}</h3>
                    <p class="text-neutral-500">{project.role}</p>
                </div>
                <span class="text-neutral-500">{project.techs.join(" · ")}</span>
            </div>
            <p class="max-w-85/100 text-sm">{project.description}</p>
            <div class="flex gap-2 my-4">
                {project
                    .topics
                    .into_iter()
                    .map(|topic| {
                        view! {
                            <span class="border border-accent-500 rounded-md text-accent-500 px-3 py-1 text-xs">
                                {topic}
                            </span>
                        }
                    })
                    .collect_view()}
            </div>
            <a
                rel="external noopener noreferrer"
                target="_blank"
                href=project.github_link
                class="border border-line rounded-md px-3 py-1 text-xs hover:bg-surface hover:text-accent-200"
            >
                {project
                    .github_link
                    .strip_prefix("https://")
                    .unwrap_or(&project.github_link)
                    .to_string()}
                " ➜ "
            </a>
        </div>
    }
}

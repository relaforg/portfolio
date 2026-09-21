use crate::{contact::LINKS, i18n::use_i18n, projects::get_projects};
use leptos::{
    ev::{KeyboardEvent, SubmitEvent},
    html::{Div, Input},
    prelude::*,
};
use leptos_i18n::{t, t_string};

#[derive(Clone)]
enum Command {
    Clear,
    Fetch,
    NotFound(String),
    InvalidArgs(String),
    Contact,
    Projects,
    Cat(String),
    Help,
}

impl Command {
    pub fn parse(str: &str) -> Self {
        let command: Vec<&str> = str.split_whitespace().collect();
        match command.as_slice() {
            ["clear", ..] => Self::Clear,
            ["fetch", ..] => Self::Fetch,
            ["contact", ..] => Self::Contact,
            ["projects", ..] => Self::Projects,
            ["cat", name, ..] => Self::Cat(name.to_string()),
            ["help", ..] => Self::Help,
            [cmd @ "cat"] => Self::InvalidArgs(cmd.to_string()),
            _ => Self::NotFound(str.to_string()),
        }
    }
}

#[derive(Clone)]
struct Entry {
    id: usize,
    input: String,
    command: Command,
}

#[component]
pub fn Terminal() -> impl IntoView {
    let mut id: usize = 1;
    let input_ref = NodeRef::<Input>::new();
    let output = NodeRef::<Div>::new();
    let command = signal(String::new());
    let (entries, set_entries) = signal(Vec::<Entry>::new());
    let (history, set_history) = signal(Vec::<String>::new());
    let (cursor, set_cursor) = signal(None::<usize>);
    set_entries.update(|v| {
        v.push(Entry {
            id: 0,
            input: command.0.get_untracked(),
            command: Command::Fetch,
        })
    });

    let command_handler = move |ev: SubmitEvent| {
        ev.prevent_default();
        if command.0.get().trim().is_empty() {
            return;
        }

        set_history.update(|v| v.push(command.0.get().trim().to_string()));

        match Command::parse(&command.0.get()) {
            Command::Clear => set_entries.update(|v| v.clear()).into_any().into_any(),
            cmd => set_entries
                .update(|v| {
                    v.push(Entry {
                        id: id,
                        input: command.0.get(),
                        command: cmd,
                    })
                })
                .into_any(),
        };
        id += 1;
        command.1.set(String::new());
    };

    let focus_on_click = move |_| {
        if let Some(node) = input_ref.get() {
            let _ = node.focus();
        }
    };

    let history = move |ev: KeyboardEvent| {
        let len = history.read().len();
        if len == 0 {
            return;
        }

        let next = match ev.key().as_str() {
            "ArrowUp" => match cursor.get() {
                None => Some(len - 1),
                Some(n) => Some(n.saturating_sub(1)),
            },
            "ArrowDown" => match cursor.get() {
                None => return,
                Some(n) if n + 1 < len => Some(n + 1),
                Some(_) => None,
            },
            _ => return,
        };

        ev.prevent_default();
        set_cursor.set(next);

        *command.1.write() = match cursor.get() {
            None => String::new(),
            Some(n) => history.get()[n].clone(),
        }
    };

    Effect::new(move |_| {
        entries.track();
        if let Some(node) = output.get() {
            node.set_scroll_top(node.scroll_height());
        }
    });

    Effect::new(move |_| {
        if let Some(node) = input_ref.get() {
            let _ = node.focus();
        }
    });

    view! {
        <div
            on:click=focus_on_click
            class="h-150 border-2 border-line rounded-md bg-linear-to-t from-bg to-surface from-70% w-full flex flex-col"
        >
            <TerminalHeader />
            <div node_ref=output class="min-h-0 overflow-y-auto">
                <TerminalContent entries=entries />
            </div>
            <div class="mx-5 my-2 flex items-center shrink-0">
                <span class="text-accent-500">"➜ ~"</span>
                <form on:submit=command_handler class="mx-2 flex-1">
                    <input
                        node_ref=input_ref
                        type="text"
                        class="focus:outline-hidden w-full"
                        bind:value=command
                        on:keydown=history
                    />
                </form>
                <div class="w-1.5 h-3 bg-accent-500 animate-blink"></div>
            </div>
        </div>
    }
}

#[component]
fn TerminalContent(entries: ReadSignal<Vec<Entry>>) -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <For
            each=move || entries.get()
            key=|entry| entry.id
            children=move |entry| {
                let Entry { input, command, .. } = entry;
                let prompt = (!input.is_empty())
                    .then(|| {

                        view! {
                            <p class="text-accent-500">
                                "➜ ~ "<span class="text-fg">{input}</span>
                            </p>
                        }
                    });
                view! {
                    <div class="mx-5 text-sm">
                        <p class="text-base my-1">{prompt}</p>
                        {match command {
                            Command::Clear => ().into_any(),
                            Command::NotFound(c) => {
                                view! { <p>{t!(i18n, commands.not_found, command = c)}</p> }
                                    .into_any()
                            }
                            Command::Fetch => view! { <Fetch /> }.into_any(),
                            Command::Contact => view! { <Contact /> }.into_any(),
                            Command::Projects => view! { <Projects /> }.into_any(),
                            Command::Cat(arg) => view! { <Cat project=arg /> }.into_any(),
                            Command::Help => view! { <Help /> }.into_any(),
                            Command::InvalidArgs(c) => {
                                view! { <p>{t!(i18n, commands.invalid_arg, command = c)}</p> }
                                    .into_any()
                            }
                        }}
                    </div>
                }
            }
        />
    }
}

#[component]
fn TerminalHeader() -> impl IntoView {
    view! {
        <div class="h-10 border-b border-line bg-surface rounded-t-[inherit] flex items-center gap-2 px-3 shrink-0">
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

#[component]
fn AsciiArt() -> impl IntoView {
    let ascii_art = [
        [1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0],
        [1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1],
    ];
    view! {
        <div>
            {move || {
                ascii_art
                    .map(|line| {
                        view! {
                            <div class="flex">
                                {move || {
                                    line.map(|c| match c {
                                        1 => view! { <div class="h-4 w-2 bg-accent-500"></div> },
                                        _ => view! { <div class="h-4 w-2 bg-transparent"></div> },
                                    })
                                }}
                            </div>
                        }
                    })
            }}
        </div>
    }
}

#[component]
fn Fetch() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="flex gap-6 items-end">
            <AsciiArt />
            <div class="grid grid-cols-[auto_1fr] gap-x-10 text-sm">
                {move || {
                    [
                        (
                            t_string!(i18n, commands.fetch.user.label),
                            t_string!(i18n, commands.fetch.user.value),
                        ),
                        (
                            t_string!(i18n, commands.fetch.title.label),
                            t_string!(i18n, commands.fetch.title.value),
                        ),
                        (
                            t_string!(i18n, commands.fetch.langs.label),
                            t_string!(i18n, commands.fetch.langs.value),
                        ),
                        (
                            t_string!(i18n, commands.fetch.editor.label),
                            t_string!(i18n, commands.fetch.editor.value),
                        ),
                        (
                            t_string!(i18n, commands.fetch.os.label),
                            t_string!(i18n, commands.fetch.os.value),
                        ),
                        (
                            t_string!(i18n, commands.fetch.city.label),
                            t_string!(i18n, commands.fetch.city.value),
                        ),
                    ]
                        .map(|(label, value)| {
                            view! {
                                <p>{label}</p>
                                <p>{value}</p>
                            }
                        })
                }}
            </div>
        </div>
        <p class="text-sm text-neutral-500 mt-3">{t!(i18n, commands.fetch.hint)}</p>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <div class="grid grid-cols-[auto_1fr] gap-x-10 text-sm">
            {LINKS
                .map(|l| {
                    view! {
                        <p>{l.label}</p>
                        <p>
                            {l
                                .link
                                .strip_prefix("https://")
                                .unwrap_or(l.link.strip_prefix("mailto:").unwrap_or(l.link))}
                        </p>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn Projects() -> impl IntoView {
    let projects = get_projects();

    view! {
        <div class="grid grid-cols-[auto_auto_1fr] gap-x-10 text-sm">
            {projects
                .into_iter()
                .map(|(k, p)| {
                    view! {
                        <p>{k}</p>
                        <p>{p.techs}</p>
                        <p>
                            {p
                                .github_link
                                .strip_prefix("https://")
                                .unwrap_or(&p.github_link)
                                .to_string()}
                        </p>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn Cat(project: String) -> impl IntoView {
    let i18n = use_i18n();
    let mut projects = get_projects();

    match projects.remove(project.as_str()) {
        Some(p) => view! {
            <div class="flex gap-10">
                <p class="text-accent-500">{p.name}</p>
                <p class="text-accent-500">"["{p.techs}"]"</p>
            </div>
            <p class="my-1">{p.description}</p>
            <a target="_blank" class="text-neutral-500" href=p.github_link.clone()>
                {p.github_link.strip_prefix("https://")}
            </a>
        }
        .into_any(),
        None => view! { <p>{t!(i18n, commands.cat.not_found, name = project)}</p> }.into_any(),
    }
}

#[component]
fn Help() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="grid grid-cols-[auto_1fr] gap-x-10">
            {move || {
                [
                    (
                        t_string!(i18n, commands.help.help.label),
                        t_string!(i18n, commands.help.help.value),
                    ),
                    (
                        t_string!(i18n, commands.help.clear.label),
                        t_string!(i18n, commands.help.clear.value),
                    ),
                    (
                        t_string!(i18n, commands.help.fetch.label),
                        t_string!(i18n, commands.help.fetch.value),
                    ),
                    (
                        t_string!(i18n, commands.help.contact.label),
                        t_string!(i18n, commands.help.contact.value),
                    ),
                    (
                        t_string!(i18n, commands.help.projects.label),
                        t_string!(i18n, commands.help.projects.value),
                    ),
                    (
                        t_string!(i18n, commands.help.cat.label),
                        t_string!(i18n, commands.help.cat.value),
                    ),
                ]
                    .map(|(command, description)| {
                        view! {
                            <p>{command}</p>
                            <p>{description}</p>
                        }
                    })
            }}
        </div>
    }
}

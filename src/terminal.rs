use leptos::{ev::SubmitEvent, html::Div, prelude::*, tachys::renderer::dom::Node};

#[derive(Clone)]
enum Command {
    Test,
}

#[derive(Clone)]
struct Entry {
    id: usize,
    input: String,
    command: Command,
}

#[component]
pub fn Terminal() -> impl IntoView {
    let mut id: usize = 0;
    let output = NodeRef::<Div>::new();
    let command = signal(String::new());
    let (entries, set_entries) = signal(Vec::<Entry>::new());

    let command_handler = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_entries.update(|v| {
            v.push(Entry {
                id: id,
                input: command.0.get(),
                command: Command::Test,
            })
        });
        id += 1;
    };

    Effect::new(move |_| {
        entries.track();
        if let Some(node) = output.get() {
            node.set_scroll_top(node.scroll_height());
        }
    });

    view! {
        <div class="h-150 border-2 border-line rounded-md bg-linear-to-t from-bg to-surface from-70% w-full flex flex-col">
            <TerminalHeader />
            <div node_ref=output class="min-h-0 overflow-y-auto">
                <For
                    each=move || entries.get()
                    key=|entry| entry.id
                    children=move |entry| {
                        view! {
                            <div class="mx-5 my-2">
                                <p class="text-accent-500">"➜ ~ " {entry.input}</p>
                                {match entry.command {
                                    Command::Test => view! { <Test /> },
                                }}
                            </div>
                        }
                    }
                />
            </div>
            <div class="mx-5 my-2 flex items-center shrink-0">
                <span class="text-accent-500">"➜ ~"</span>
                <form on:submit=command_handler class="mx-2 flex-1">
                    <input type="text" class="focus:outline-hidden w-full" bind:value=command />
                </form>
                <div class="w-1.5 h-3 bg-accent-500 animate-blink"></div>
            </div>
        </div>
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
fn Test() -> impl IntoView {
    view! { <p>"test"</p> }
}

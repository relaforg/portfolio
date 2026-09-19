use leptos::prelude::*;
use leptos_i18n::t;

use crate::i18n::use_i18n;

pub struct SocialLink {
    pub label: &'static str,
    pub username: &'static str,
    pub link: &'static str,
}

pub const LINKS: [SocialLink; 4] = [
    SocialLink {
        label: "email",
        username: "contact@remi-laforgue.fr",
        link: "mailto:contact@remi-laforgue.fr",
    },
    SocialLink {
        label: "github",
        username: "relaforg",
        link: "https://github.com/relaforg",
    },
    SocialLink {
        label: "linkedin",
        username: "rémi-laforgue",
        link: "https://www.linkedin.com/in/r%C3%A9mi-laforgue-8b02b242b/",
    },
    SocialLink {
        label: "cv",
        username: "cv.pdf",
        link: "test.pdf",
    },
];

#[component]
pub fn Contact() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section id="contact" class="my-3">
            <h2 class="text-neutral-500">"// CONTACT"</h2>
            <div class="grid grid-cols-2 gap-5">
                <div>
                    <h3 class="text-3xl">{t!(i18n, contact.subtitle)}</h3>
                    <p class="text-sm my-5 text-neutral-500">{t!(i18n, contact.description)}</p>
                </div>
                <div>
                    <hr class="border-line" />
                    {LINKS
                        .into_iter()
                        .map(|n| {
                            view! {
                                <a
                                    rel="external noopener noreferrer"
                                    target="_blank"
                                    href=n.link
                                    class="text-sm flex justify-between hover:text-accent-500 my-2 px-1"
                                >
                                    <span class="text-neutral-500">{n.label}</span>
                                    <span>{n.username}</span>
                                </a>
                                <hr class="border-line" />
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}

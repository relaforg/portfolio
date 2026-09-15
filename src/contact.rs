use leptos::prelude::*;

struct SocialLink {
    label: String,
    username: String,
    link: String,
}

#[component]
pub fn Contact() -> impl IntoView {
    let links = vec![
        SocialLink {
            label: "email".to_string(),
            username: "contact@remi-laforgue.fr".to_string(),
            link: "mailto:contact@remi-laforgue.fr".to_string(),
        },
        SocialLink {
            label: "github".to_string(),
            username: "relaforg".to_string(),
            link: "https://github.com/relaforg".to_string(),
        },
        SocialLink {
            label: "linkedin".to_string(),
            username: "rémi-laforgue".to_string(),
            link: "https://www.linkedin.com/in/r%C3%A9mi-laforgue-8b02b242b/".to_string(),
        },
        SocialLink {
            label: "cv".to_string(),
            username: "cv.pdf".to_string(),
            link: "".to_string(),
        },
    ];
    view! {
        <h2 id="contact" class="my-3 text-neutral-500">
            "// CONTACT"
        </h2>
        <div class="grid grid-cols-2 gap-5">
            <div>
                <h3 class="text-3xl">"Contactez-moi"</h3>
                <p class="text-sm my-5 text-neutral-500">
                    "Disponible pour des stages, et ouvert à toute discussion — un projet, une question technique, ou simplement l'envie de parler de code."
                </p>
            </div>
            <div>
                <hr class="border-line" />
                {links
                    .into_iter()
                    .map(|n| {
                        view! {
                            <a
                                rel="external"
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
    }
}

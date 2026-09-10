use leptos::prelude::*;

use crate::app::Hr;

struct Project {
    name: String,
    description: String,
    techs: Vec<String>,
    topics: Vec<String>,
    github_link: String,
    role: String,
}

fn get_projects() -> Vec<Project> {
    vec![
        Project {
            name: "The Answer Protocol".to_string(),
            role: "server".to_string(),
            description: "Un MUD : monde textuel persistant où plusieurs joueurs explorent des salles, discutent et coopèrent en temps réel. Serveur TCP gérant les connexions, protocole ligne à ligne et monde 100 % configurable en YAML. Deux clients, un disponible dans le terminal et l'autre graphiquement.".to_string(),
            techs: vec!["Rust".to_string()],
            topics: vec![
                "Réseau".to_string(),
                "Jeu".to_string(),
                "Multi-joueur".to_string(),
                "42".to_string(),
            ],
            github_link: "https://github.com/Arcanovax/TAP".to_string(),
        },
        Project {
            name: "Chip8".to_string(),
            role: "solo".to_string(),
            description: "Émulateur CHIP-8 écrit de zéro : jeu d'instructions complet, registres, pile, timers, affichage 64×32 et clavier hexadécimal via SDL2. Un drapeau bascule entre le comportement original et le comportement des interpréteurs modernes sur les opcodes ambigus.".to_string(),
            techs: vec!["Rust".to_string()],
            topics: vec!["Émulation".to_string(), "Bas niveau".to_string()],
            github_link: "https://github.com/relaforg/chip8".to_string(),
        },
        Project {
            name: "A Maze Ing".to_string(),
            role: "frontend".to_string(),
            description: "Générateur et solveur de labyrinthe : six formes (rectangle, carré, cercle, donut, losange et ellipse), résolution A*, génération reproductible par seed. Interface en deux fenêtres, le labyrinthe d'un côté, les contrôles de l'autre. La fenêtre principale est redimensionnable et permet de zoomer et de se déplacer sur le labyrinthe.".to_string(),
            techs: vec!["Python".to_string()],
            topics: vec!["Graphique".to_string(), "42".to_string()],
            github_link: "https://github.com/relaforg/a_maze_ing".to_string(),
        },
    ]
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = get_projects();
    view! {
        <h2 id="projects" class="my-3 text-neutral-500">
            "// PROJETS MIS EN AVANT"
        </h2>
        <Hr />
        {projects
            .into_iter()
            .map(|n| {
                view! {
                    <ProjectView project=n />
                    <Hr />
                }
            })
            .collect_view()}
    }
}

#[component]
fn ProjectView(project: Project) -> impl IntoView {
    view! {
        <div class="my-10">
            <div class="flex justify-between items-end my-5">
                <div class="flex items-end gap-2">
                    <h3 class="text-3xl">{project.name}</h3>
                    <p class="text-neutral-500">{project.role}</p>
                </div>
                <span class="text-neutral-500">{project.techs.join(" · ")}</span>
            </div>
            <p class="max-w-85/100">{project.description}</p>
            <div class="flex gap-2 my-4">
                {project
                    .topics
                    .into_iter()
                    .map(|topic| {
                        view! {
                            <span class="border border-accent-500 rounded-md text-accent-500 px-3 py-1 text-sm">
                                {topic}
                            </span>
                        }
                    })
                    .collect_view()}
            </div>
            <a
                rel="external"
                href=project.github_link
                class="border border-line rounded-md px-3 py-1 text-sm"
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

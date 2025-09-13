use cflookup_cli::cflookup::{
    Mod, get_file_info, get_project_by_id, get_project_by_slug, search_project_by_slug,
};
use clap::{Parser, Subcommand};
use http::Uri;
use serde::Serialize;

/// A CLI for https://cflookup.com
#[derive(Parser, Debug)]
#[command(args_conflicts_with_subcommands = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    nested: Option<Commands>,

    /// the project ID or slug
    project_id: Option<u64>,
}

#[derive(Subcommand, Clone, Debug)]
#[group(required = true, multiple = false)]
enum Commands {
    /// look up a file by ID
    #[command(arg_required_else_help = true)]
    File {
        /// the ID of the file
        file_id: u64,
    },
    /// search for a project by its slug
    #[command(arg_required_else_help = true)]
    Slug {
        /// the game name
        #[arg(short = 'g', long, default_value = "minecraft")]
        game: String,

        /// the category name
        #[arg(short = 'c', long, default_value = "mc-mods")]
        category: String,

        /// the slug to search for
        slug: String,
    },
    /// Search for a slug across all of CurseForge
    #[command(arg_required_else_help = true)]
    Search {
        /// the slug to search for
        slug: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli: Cli = Cli::parse();

    match cli.nested {
        Some(sub) => {
            match sub {
                Commands::File { file_id } => match get_file_info(file_id).await? {
                    Some(json) => serde_yaml_bw::to_writer(std::io::stdout(), &json)?,
                    None => eprintln!("No file found for ID {file_id}"),
                },
                Commands::Slug {
                    game,
                    category,
                    slug,
                } => {
                    match get_project_by_slug(&game, &category, &slug).await? {
                        Some(json) => serde_yaml_bw::to_writer(std::io::stdout(), &json)?,
                        None => {
                            // no project found but maybe it's an ID?
                            if let Ok(id) = slug.parse::<u64>()
                                && let Some(json) = get_project_by_id(id).await?
                            {
                                serde_yaml_bw::to_writer(std::io::stdout(), &json)?
                            } else {
                                eprintln!("No project found for {game}/{category}/{slug}");
                            }
                        }
                    }
                }
                Commands::Search { slug } => {
                    let (categories, total) = search_project_by_slug(&slug).await?;
                    if total == 0 {
                        eprintln!("No projects found for slug {slug}!");
                        return Ok(());
                    }

                    let projects_str = match total {
                        1 => "project",
                        _ => "projects",
                    };
                    let categories_str = match categories.len() {
                        1 => "category",
                        _ => "categories",
                    };

                    eprintln!(
                        "Found {total} {projects_str} across {category_count} {categories_str}:",
                        category_count = categories.len()
                    );

                    #[derive(Debug, Serialize)]
                    struct ResultMapping {
                        game: String,
                        category: String,
                        #[serde(with = "http_serde::option::uri")]
                        url: Option<Uri>,
                        mods: Vec<Mod>,
                    }
                    let results: Vec<ResultMapping> = categories
                        .iter()
                        .map(|(category, entries)| ResultMapping {
                            game: category.game.clone(),
                            category: category.category.clone(),
                            url: category.url.clone(),
                            mods: entries.clone(),
                        })
                        .collect();

                    if results.len() == 1 {
                        serde_yaml_bw::to_writer(std::io::stdout(), results.first().unwrap())?
                    } else {
                        serde_yaml_bw::to_writer(std::io::stdout(), &results)?
                    }
                }
            }
        }
        None => {
            let id = cli.project_id.expect("Missing project_id");
            match get_project_by_id(id).await? {
                Some(json) => serde_yaml_bw::to_writer(std::io::stdout(), &json)?,
                None => eprintln!("No project found for ID {id}"),
            }
        }
    }

    Ok(())
}

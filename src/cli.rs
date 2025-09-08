use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommand,

    #[clap(flatten)]
    pub config_args: ConfigArgs,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    /// Initialize a new modpack project
    Init(InitArgs),

    /// Import data from a Modrinth modpack to the existing Packrinth modpack
    Import(ImportArgs),

    /// Add or remove Modrinth projects and tweak them for your branches
    Project(ProjectArgs),

    /// Create and remove branches that separate your Modpack for various versions
    Branch(BranchArgs),

    /// Update branches with the newest project versions
    Update(UpdateArgs),

    /// Export a branch to a Modrinth modpack
    Export(ExportArgs),

    /// Generate Markdown documentation
    Doc(DocArgs),

    /// Generate shell completion for Packrinth
    Completions(CompletionsArgs),

    /// Show information about the current Packrinth installation
    Version(VersionArgs),
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    /// Set the root directory of the modpack (directory of modpack.json)
    #[clap(short, long, global = true)]
    pub directory: Option<PathBuf>,

    /// Output more information about the current process
    #[clap(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Debug, Parser)]
pub struct InitArgs {
    /// Don't initialize a Git repository
    #[clap(short = 'G', long)]
    pub no_git_repo: bool,

    /// Force initializing a new modpack even if one already exists
    #[clap(short, long)]
    pub force: bool,
}

#[derive(Debug, Parser)]
pub struct ImportArgs {
    /// Location of the Modrinth modpack to import
    pub modrinth_pack: PathBuf,

    /// Add projects to the modpack configuration file if they aren't in there yet
    #[clap(short = 'p', long)]
    pub add_projects: bool,

    /// Force importing a modpack even if the branch already exists (the branch will be overwritten)
    #[clap(short, long)]
    pub force: bool,
}

#[derive(Debug, Parser)]
pub struct ProjectArgs {
    #[clap(subcommand)]
    pub command: Option<ProjectSubCommand>,

    /// List information about added projects. If none are specified, all projects will be listed.
    pub projects: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
pub enum ProjectSubCommand {
    /// List all projects that are currently added to this modpack
    #[clap(visible_alias = "ls")]
    List(ListProjectsArgs),

    /// Add projects to this modpack
    Add(AddProjectsArgs),

    /// Add a version override to a project in this modpack
    VersionOverride(VersionOverrideProjectArgs),

    /// Add inclusions to a project in this modpack
    Inclusions(InclusionsProjectArgs),

    /// Add exclusions to a project in this modpack
    Exclusions(ExclusionsProjectArgs),

    /// Remove projects from this modpack
    #[clap(visible_alias = "rm")]
    Remove(RemoveProjectsArgs),
}

#[derive(Parser, Debug)]
pub struct ListProjectsArgs;

#[derive(Parser, Debug)]
pub struct AddProjectsArgs {
    // Allow so we don't have to put the slug between `
    #[allow(clippy::doc_markdown)]
    /// Projects to add
    ///
    /// The projects must be from Modrinth. You have to specify either the human-readable
    /// slug that appears in the URL (fabric-api) or the slug (P7dR8mSH).
    #[arg(required = true)]
    pub projects: Vec<String>,

    /// Add branch inclusions for the projects that you are adding
    ///
    /// The added projects will only be updated for the branches you specify.
    /// For a project, you can only have inclusions OR exclusions.
    #[clap(short, long, group = "include_or_exclude")]
    pub inclusions: Option<Vec<String>>,

    /// Add branch exclusions for the projects that you are adding
    ///
    /// The added projects will not be updated for the branches you specify,
    /// but the unspecified branches will be updated with this project.
    /// For a project, you can only have inclusions OR exclusions.
    #[clap(short, long, group = "include_or_exclude")]
    pub exclusions: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
pub struct VersionOverrideProjectArgs {
    #[clap(subcommand)]
    pub command: VersionOverrideSubCommand,
}

#[derive(Parser, Debug)]
pub enum VersionOverrideSubCommand {
    /// Add a version override to a project
    Add(AddVersionOverrideArgs),

    /// Remove a version override from a project
    #[clap(visible_alias = "rm")]
    Remove(RemoveVersionOverrideArgs),
}

#[derive(Parser, Debug)]
pub struct AddVersionOverrideArgs {
    /// Project to add the version override to
    pub project: String,

    /// Branch that you want to be overridden
    pub branch: String,

    // Allow so we don't have to put the slug between `
    #[allow(clippy::doc_markdown)]
    /// The version ID of the override
    ///
    /// This must be a Modrinth version ID. You can find this by going to a project on the
    /// Modrinth website, navigating to the version that you want to override and copying
    /// the version ID that looks something like this: Q8ssLFZp
    pub project_version_id: String,
}

#[derive(Parser, Debug)]
pub struct RemoveVersionOverrideArgs {
    /// Project to remove the override from
    pub project: String,

    /// Branch to remove the override from
    pub branch: Option<String>,

    /// Remove all overrides from a project
    #[clap(short, long)]
    pub all: bool,
}

#[derive(Parser, Debug)]
pub struct InclusionsProjectArgs {
    #[clap(subcommand)]
    pub command: InclusionsSubCommand,
}

#[derive(Parser, Debug)]
pub enum InclusionsSubCommand {
    /// Add inclusions to a project
    Add(AddInclusionsArgs),

    /// Remove inclusions from a project
    #[clap(visible_alias = "rm")]
    Remove(RemoveInclusionsArgs),
}

#[derive(Parser, Debug)]
pub struct AddInclusionsArgs {
    /// Project to add inclusions to
    pub project: String,

    /// Branches to include
    #[arg(required = true)]
    pub inclusions: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct RemoveInclusionsArgs {
    /// Project to remove inclusions from
    pub project: String,

    /// Inclusions to remove
    pub inclusions: Option<Vec<String>>,

    /// Remove all inclusions from the project
    #[clap(short, long)]
    pub all: bool,
}

#[derive(Parser, Debug)]
pub struct ExclusionsProjectArgs {
    #[clap(subcommand)]
    pub command: ExclusionsSubCommand,
}

#[derive(Parser, Debug)]
pub enum ExclusionsSubCommand {
    /// Add exclusions to a project
    Add(AddExclusionsArgs),

    #[clap(visible_alias = "rm")]
    /// Remove exclusions from a project
    Remove(RemoveExclusionsArgs),
}

#[derive(Parser, Debug)]
pub struct AddExclusionsArgs {
    /// Project to add exclusions to
    pub project: String,

    /// Branches to exclude
    #[arg(required = true)]
    pub exclusions: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct RemoveExclusionsArgs {
    /// Project to remove exclusions from
    pub project: String,

    /// Exclusions to remove
    pub exclusions: Option<Vec<String>>,

    /// Remove all exclusions from the project
    #[clap(short, long)]
    pub all: bool,
}

#[derive(Parser, Debug)]
pub struct RemoveProjectsArgs {
    /// Projects to remove from the modpack
    #[arg(required = true)]
    pub projects: Vec<String>,
}

// Allow because it is just a CLI.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
pub struct UpdateArgs {
    /// Branches to update. If no branches are specified, all branches will be updated.
    pub branches: Option<Vec<String>>,

    /// Don't allow alpha releases to be added to branch files
    #[clap(long)]
    pub no_alpha: bool,

    /// Don't allow beta releases to be added to branch files
    #[clap(long)]
    pub no_beta: bool,

    /// For every environment (server and client), set all projects as required
    #[clap(short, long)]
    pub require_all: bool,

    /// Automatically add any dependencies required by the projects in the modpack
    #[clap(short, long)]
    pub auto_dependencies: bool,

    /// If the modpack is in a Git repository, allow updating even if there are uncommitted changes
    #[clap(short = 'D', long)]
    pub allow_dirty: bool,
}

#[derive(Debug, Parser)]
pub struct BranchArgs {
    #[clap(subcommand)]
    pub command: Option<BranchSubCommand>,

    /// Branches to list. If none are specified, you must use a subcommand.
    pub branches: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
pub enum BranchSubCommand {
    /// List information about all branches
    #[clap(visible_alias = "ls")]
    List(ListBranchesArgs),

    /// Add new branches
    Add(AddBranchesArgs),

    /// Remove branches
    #[clap(visible_alias = "rm")]
    Remove(RemoveBranchesArgs),
}

#[derive(Parser, Debug)]
pub struct ListBranchesArgs;

#[derive(Parser, Debug)]
pub struct AddBranchesArgs {
    /// Names of new branches to add
    #[arg(required = true)]
    pub branches: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct RemoveBranchesArgs {
    /// Names of branches to remove
    #[arg(required = true)]
    pub branches: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct ExportArgs {
    /// Branches to export. If no branches are specified, all branches will be exported.
    pub branches: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
pub struct DocArgs;

#[derive(Parser, Debug)]
pub struct CompletionsArgs {
    /// The shell to generate the completion for
    pub shell: CompletionShell,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum CompletionShell {
    Bash,
    Elvish,
    Fish,

    #[clap(name = "powershell")]
    PowerShell,

    Zsh,
}

#[derive(Parser, Debug)]
pub struct VersionArgs;

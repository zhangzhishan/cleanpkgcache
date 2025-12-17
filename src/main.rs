use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

const ROO_TASK_PATHS: [&str; 2] = [
    r"C:\Users\zhizha\AppData\Roaming\Code\User\globalStorage\microsoftai.ms-roo-cline\tasks",
    r"C:\Users\zhizha\AppData\Roaming\Code\User\globalStorage\rooveterinaryinc.roo-cline\tasks",
];
const TWO_MONTHS_IN_SECONDS: u64 = 60 * 24 * 60 * 60;

#[derive(Parser)]
#[command(name = "cleanpkgcache")]
#[command(about = "Clean package cache by keeping only the latest 2 versions of each package")]
#[command(version = "0.2.1")]
struct Args {
    /// Path to the package cache directory
    #[arg(default_value = r"C:\PkgCache\VC17LTCG")]
    path: PathBuf,

    /// Dry run - show what would be deleted without actually deleting
    #[arg(short, long)]
    dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Also clean Roo checkpoints older than 2 months
    #[arg(long)]
    clean_roo_checkpoints: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.dry_run {
        println!("DRY RUN MODE - No files will be deleted");
    }

    // Only clean package cache if path exists or if not running roo-only mode
    let should_clean_packages = args.path.exists() && args.path.is_dir();

    if should_clean_packages {
        println!("Cleaning package cache at: {}", args.path.display());
        clean_package_cache(&args.path, args.dry_run, args.verbose)?;
    } else if !args.clean_roo_checkpoints {
        // Only error out if we're not cleaning roo checkpoints either
        if !args.path.exists() {
            anyhow::bail!("Path does not exist: {}", args.path.display());
        }
        if !args.path.is_dir() {
            anyhow::bail!("Path is not a directory: {}", args.path.display());
        }
    }

    if args.clean_roo_checkpoints {
        clean_roo_checkpoints(args.dry_run, args.verbose)?;
    }

    Ok(())
}

fn clean_package_cache(cache_path: &Path, dry_run: bool, verbose: bool) -> Result<()> {
    let mut packages: HashMap<String, Vec<PackageVersion>> = HashMap::new();

    // First pass: collect all package directories and their versions
    for entry in fs::read_dir(cache_path)
        .with_context(|| format!("Failed to read directory: {}", cache_path.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let package_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();

        if package_name.is_empty() {
            continue;
        }

        // Collect all version directories for this package
        let mut versions = Vec::new();

        for version_entry in fs::read_dir(&path)
            .with_context(|| format!("Failed to read package directory: {}", path.display()))?
        {
            let version_entry = version_entry?;
            let version_path = version_entry.path();

            if !version_path.is_dir() {
                continue;
            }

            let version_name = version_path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_string();

            if version_name.is_empty() {
                continue;
            }

            // Get modification time for sorting
            let metadata = fs::metadata(&version_path)
                .with_context(|| format!("Failed to get metadata for: {}", version_path.display()))?;

            let modified = metadata.modified()
                .with_context(|| format!("Failed to get modification time for: {}", version_path.display()))?;

            versions.push(PackageVersion {
                name: version_name,
                path: version_path,
                modified,
            });
        }

        if !versions.is_empty() {
            packages.insert(package_name, versions);
        }
    }

    // Second pass: clean each package
    let mut total_deleted = 0;
    let mut total_kept = 0;
    let packages_count = packages.len();

    for (package_name, mut versions) in packages {
        // Sort versions by modification time (newest first)
        versions.sort_by(|a, b| b.modified.cmp(&a.modified));

        if verbose {
            println!("\nPackage: {}", package_name);
            println!("  Found {} versions:", versions.len());
            for (i, version) in versions.iter().enumerate() {
                println!("    {}: {} (modified: {:?})",
                    i + 1,
                    version.name,
                    version.modified
                );
            }
        }

        // Keep the latest 2 versions, delete the rest
        let to_keep = versions.iter().take(2);
        let to_delete = versions.iter().skip(2);

        for version in to_keep {
            if verbose {
                println!("  Keeping: {}", version.name);
            }
            total_kept += 1;
        }

        for version in to_delete {
            if dry_run {
                println!("  Would delete: {}", version.path.display());
            } else {
                println!("  Deleting: {}", version.path.display());
                fs::remove_dir_all(&version.path)
                    .with_context(|| format!("Failed to delete directory: {}", version.path.display()))?;
            }
            total_deleted += 1;
        }
    }

    println!("\nSummary:");
    println!("  Packages processed: {}", packages_count);
    println!("  Versions kept: {}", total_kept);
    if dry_run {
        println!("  Versions that would be deleted: {}", total_deleted);
    } else {
        println!("  Versions deleted: {}", total_deleted);
    }

    Ok(())
}

fn clean_roo_checkpoints(dry_run: bool, verbose: bool) -> Result<()> {
    let two_months = Duration::from_secs(TWO_MONTHS_IN_SECONDS);
    let now = SystemTime::now();
    let mut tasks_checked = 0;
    let mut checkpoints_targets = 0;

    println!("\nCleaning Roo checkpoints older than approximately 2 months...");

    for base_path in ROO_TASK_PATHS {
        let base_dir = Path::new(base_path);

        if !base_dir.exists() {
            if verbose {
                println!("  Skipping {} (path not found)", base_dir.display());
            }
            continue;
        }

        for entry in fs::read_dir(base_dir)
            .with_context(|| format!("Failed to read Roo tasks directory: {}", base_dir.display()))?
        {
            let entry = entry?;
            let task_path = entry.path();

            if !task_path.is_dir() {
                continue;
            }
            tasks_checked += 1;

            let metadata = fs::metadata(&task_path)
                .with_context(|| format!("Failed to read metadata for task: {}", task_path.display()))?;
            let modified = metadata.modified()
                .with_context(|| format!("Failed to get modification time for task: {}", task_path.display()))?;
            let age = now.duration_since(modified).unwrap_or(Duration::ZERO);

            if age < two_months {
                if verbose {
                    println!("  Keeping checkpoints for {} (age < 2 months)", task_path.display());
                }
                continue;
            }

            let checkpoints_path = task_path.join("checkpoints");
            if !checkpoints_path.exists() {
                continue;
            }

            if dry_run {
                println!("  Would delete checkpoints: {}", checkpoints_path.display());
            } else {
                println!("  Deleting checkpoints: {}", checkpoints_path.display());
                fs::remove_dir_all(&checkpoints_path).with_context(|| {
                    format!("Failed to delete checkpoints directory: {}", checkpoints_path.display())
                })?;
            }
            checkpoints_targets += 1;
        }
    }

    println!("Roo checkpoints summary:");
    println!("  Task folders inspected: {}", tasks_checked);
    if dry_run {
        println!("  Checkpoints eligible for deletion: {}", checkpoints_targets);
    } else {
        println!("  Checkpoints deleted: {}", checkpoints_targets);
    }

    Ok(())
}

#[derive(Debug)]
struct PackageVersion {
    name: String,
    path: PathBuf,
    modified: SystemTime,
}

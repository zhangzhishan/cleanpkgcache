# CleanPkgCache

A Rust CLI tool to clean package cache directories by keeping only the latest 2 versions of each package.

## Overview

This tool is designed to clean package cache folders where each subfolder represents a package, and each package folder contains multiple versions. The tool will identify all packages and their versions, then remove older versions while keeping only the latest 2 versions of each package.

## Features

- **Default Path**: Automatically targets `C:\PkgCache\VC17LTCG` if no path is specified
- **Dry Run Mode**: Preview what would be deleted without actually deleting files
- **Verbose Output**: Detailed information about packages and versions found
- **Safe Deletion**: Only removes directories that are clearly version folders within package directories
- **Roo Checkpoint Cleanup**: Optional flag to remove outdated MS Roo Code task checkpoints (older than ~2 months)
- **Summary Report**: Shows how many packages were processed and versions kept/deleted

## Installation

1. Make sure you have Rust installed on your system
2. Clone or download this project
3. Build the release version:
   ```bash
   cargo build --release
   ```
4. The executable will be available at `target/release/cleanpkgcache.exe`

## Usage

### Basic Usage
```bash
# Use default path (C:\PkgCache\VC17LTCG)
cleanpkgcache.exe

# Specify a custom path
cleanpkgcache.exe "C:\Your\Custom\Cache\Path"
```

### Options

```bash
# Dry run - see what would be deleted without actually deleting
cleanpkgcache.exe --dry-run

# Verbose output - see detailed information about packages and versions
cleanpkgcache.exe --verbose

# Combine options
cleanpkgcache.exe --dry-run --verbose "C:\Your\Cache\Path"

# Include Roo checkpoint cleanup
cleanpkgcache.exe --clean-roo-checkpoints --dry-run
```

### Command Line Arguments

- `PATH` - Path to the package cache directory (optional, defaults to `C:\PkgCache\VC17LTCG`)
- `-d, --dry-run` - Show what would be deleted without actually deleting
- `-v, --verbose` - Show detailed output about packages and versions
- `--clean-roo-checkpoints` - Also clean checkpoints under the MS Roo Code and Roo Code Extension `tasks` folders that are older than ~2 months
- `-h, --help` - Show help information
- `-V, --version` - Show version information

## Example Output

```
Cleaning package cache at: C:\PkgCache\VC17LTCG

Package: SomePackage
  Found 5 versions:
    1: v1.2.3 (modified: 2023-12-01T10:00:00Z)
    2: v1.2.2 (modified: 2023-11-15T14:30:00Z)
    3: v1.2.1 (modified: 2023-11-01T09:15:00Z)
    4: v1.2.0 (modified: 2023-10-15T16:45:00Z)
    5: v1.1.9 (modified: 2023-10-01T11:20:00Z)
  Keeping: v1.2.3
  Keeping: v1.2.2
  Deleting: C:\PkgCache\VC17LTCG\SomePackage\v1.2.1
  Deleting: C:\PkgCache\VC17LTCG\SomePackage\v1.2.0
  Deleting: C:\PkgCache\VC17LTCG\SomePackage\v1.1.9

Summary:
  Packages processed: 1
  Versions kept: 2
  Versions deleted: 3
```

## How It Works

1. **Discovery**: The tool scans the specified cache directory for package folders
2. **Version Detection**: For each package, it finds all version subdirectories
3. **Sorting**: Versions are sorted by modification time (newest first)
4. **Cleanup**: Keeps the 2 most recent versions and deletes the rest
5. **Reporting**: Provides a summary of the cleanup operation
6. **Optional Roo Cleanup**: When `--clean-roo-checkpoints` is passed, the tool also scans `C:\Users\zhizha\AppData\Roaming\Code\User\globalStorage\microsoftai.ms-roo-cline\tasks` and `C:\Users\zhizha\AppData\Roaming\Code\User\globalStorage\rooveterinaryinc.roo-cline\tasks`, deleting `checkpoints` folders for tasks older than roughly two months

## Safety Features

- **Path Validation**: Ensures the specified path exists and is a directory
- **Dry Run Mode**: Allows you to preview changes before applying them
- **Error Handling**: Graceful handling of permission errors and invalid paths
- **Detailed Logging**: Clear output about what is being kept and what is being deleted

## Requirements

- Windows (designed for Windows paths, but should work on other platforms)
- Rust 1.70+ (for building from source)

## License

MIT License

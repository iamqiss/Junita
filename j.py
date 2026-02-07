#!/usr/bin/env python3
"""
Junita Rebranding Script
========================
Rebrand Blinc to Junita across the entire codebase.

This script will:
- Replace all variations of "Blinc" with "Junita"
- Update GitHub username from @project-blinc to @iamqiss
- Handle case variations (blinc, Blinc, BLINC)
- Update URLs, package names, and documentation
- Preserve file permissions and structure

Usage:
    python3 rebrand.py /path/to/junita/repo
"""

import os
import sys
import re
import fnmatch
from pathlib import Path
from typing import List, Dict, Tuple

# Rebranding mappings
REBRAND_PATTERNS = [
    # Exact case matches
    ('Blinc', 'Junita'),
    ('blinc', 'junita'),
    ('BLINC', 'JUNITA'),
    
    # GitHub usernames and organizations
    ('project-blinc', 'iamqiss'),
    ('project_blinc', 'iamqiss'),
    ('@project-blinc', '@iamqiss'),
    
    # URLs and domains
    ('https://github.com/project-blinc', 'https://github.com/iamqiss'),
    ('https://project-blinc.github.io', 'https://iamqiss.github.io'),
    ('project-blinc.github.io', 'iamqiss.github.io'),
    
    # Package names (common in Rust/Cargo)
    ('blinc-', 'junita-'),
    ('blinc_', 'junita_'),
]

# Files to skip (binary, generated, or special files)
SKIP_PATTERNS = [
    '*.png', '*.jpg', '*.jpeg', '*.gif', '*.ico', '*.svg',
    '*.woff', '*.woff2', '*.ttf', '*.eot',
    '*.mp4', '*.webm', '*.ogg',
    '*.zip', '*.tar', '*.gz', '*.bz2',
    '*.exe', '*.dll', '*.so', '*.dylib',
    '.git/*', '.git/**/*',
    'target/*', 'target/**/*',  # Rust build directory
    'node_modules/*', 'node_modules/**/*',
    '*.lock',  # Don't modify lock files
]

# Directories to skip entirely
SKIP_DIRS = {'.git', 'target', 'node_modules', 'dist', 'build', '.cache'}

def should_skip_file(filepath: Path) -> bool:
    """Check if file should be skipped based on patterns."""
    filepath_str = str(filepath)
    
    # Check if in skip directory
    for part in filepath.parts:
        if part in SKIP_DIRS:
            return True
    
    # Check against skip patterns
    for pattern in SKIP_PATTERNS:
        if fnmatch.fnmatch(filepath_str, pattern):
            return True
    
    return False

def is_binary_file(filepath: Path) -> bool:
    """Check if file is binary by reading first few bytes."""
    try:
        with open(filepath, 'rb') as f:
            chunk = f.read(1024)
            # Check for null bytes (common in binary files)
            if b'\x00' in chunk:
                return True
            # Try to decode as text
            try:
                chunk.decode('utf-8')
                return False
            except UnicodeDecodeError:
                return True
    except Exception:
        return True

def rebrand_content(content: str) -> Tuple[str, int]:
    """
    Apply all rebranding patterns to content.
    Returns (new_content, number_of_changes)
    """
    new_content = content
    total_changes = 0
    
    for old, new in REBRAND_PATTERNS:
        # Count occurrences before replacement
        count = new_content.count(old)
        if count > 0:
            new_content = new_content.replace(old, new)
            total_changes += count
    
    return new_content, total_changes

def rebrand_file(filepath: Path, dry_run: bool = False) -> Dict:
    """
    Rebrand a single file.
    Returns dict with file info and changes made.
    """
    result = {
        'path': str(filepath),
        'processed': False,
        'changes': 0,
        'error': None
    }
    
    try:
        # Read file content
        with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
            original_content = f.read()
        
        # Apply rebranding
        new_content, changes = rebrand_content(original_content)
        
        result['changes'] = changes
        
        # Write back if changes were made and not dry run
        if changes > 0 and not dry_run:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
            result['processed'] = True
        elif changes > 0:
            result['processed'] = True  # Would be processed
        
    except Exception as e:
        result['error'] = str(e)
    
    return result

def rename_paths(root_path: Path, dry_run: bool = False) -> List[Tuple[Path, Path]]:
    """
    Rename files and directories containing 'blinc' to 'junita'.
    Returns list of (old_path, new_path) tuples.
    """
    renames = []
    
    # Walk directory tree bottom-up so we rename files before directories
    for dirpath, dirnames, filenames in os.walk(root_path, topdown=False):
        current_dir = Path(dirpath)
        
        # Skip special directories
        if any(skip_dir in current_dir.parts for skip_dir in SKIP_DIRS):
            continue
        
        # Rename files
        for filename in filenames:
            if 'blinc' in filename.lower():
                old_path = current_dir / filename
                new_filename = filename
                
                # Apply all variations
                for old, new in REBRAND_PATTERNS:
                    if old in new_filename:
                        new_filename = new_filename.replace(old, new)
                
                new_path = current_dir / new_filename
                
                if old_path != new_path:
                    renames.append((old_path, new_path))
                    if not dry_run:
                        old_path.rename(new_path)
        
        # Rename directories
        for dirname in dirnames:
            if 'blinc' in dirname.lower():
                old_path = current_dir / dirname
                new_dirname = dirname
                
                # Apply all variations
                for old, new in REBRAND_PATTERNS:
                    if old in new_dirname:
                        new_dirname = new_dirname.replace(old, new)
                
                new_path = current_dir / new_dirname
                
                if old_path != new_path:
                    renames.append((old_path, new_path))
                    if not dry_run:
                        old_path.rename(new_path)
    
    return renames

def find_files_to_process(root_path: Path) -> List[Path]:
    """Find all text files that need processing."""
    files_to_process = []
    
    for filepath in root_path.rglob('*'):
        # Skip if not a file
        if not filepath.is_file():
            continue
        
        # Skip based on patterns
        if should_skip_file(filepath):
            continue
        
        # Skip binary files
        if is_binary_file(filepath):
            continue
        
        files_to_process.append(filepath)
    
    return files_to_process

def print_summary(results: List[Dict], renames: List[Tuple[Path, Path]]):
    """Print summary of rebranding operation."""
    total_files = len(results)
    processed_files = sum(1 for r in results if r['processed'])
    total_changes = sum(r['changes'] for r in results)
    errors = [r for r in results if r['error']]
    
    print("\n" + "="*60)
    print("REBRANDING SUMMARY")
    print("="*60)
    print(f"Total files scanned: {total_files}")
    print(f"Files modified: {processed_files}")
    print(f"Total replacements: {total_changes}")
    print(f"Paths renamed: {len(renames)}")
    
    if errors:
        print(f"\nErrors encountered: {len(errors)}")
        for err in errors[:5]:  # Show first 5 errors
            print(f"  - {err['path']}: {err['error']}")
        if len(errors) > 5:
            print(f"  ... and {len(errors) - 5} more")
    
    # Show files with most changes
    top_changes = sorted(results, key=lambda x: x['changes'], reverse=True)[:10]
    if top_changes[0]['changes'] > 0:
        print("\nTop files by number of changes:")
        for r in top_changes:
            if r['changes'] > 0:
                print(f"  {r['changes']:4d} changes - {r['path']}")
    
    # Show renamed paths
    if renames:
        print("\nRenamed paths:")
        for old, new in renames[:10]:
            print(f"  {old.name} → {new.name}")
        if len(renames) > 10:
            print(f"  ... and {len(renames) - 10} more")
    
    print("="*60)

def main():
    """Main rebranding function."""
    if len(sys.argv) < 2:
        print("Usage: python3 rebrand.py <path-to-repo> [--dry-run]")
        print("\nThis script will rebrand Blinc to Junita throughout the repository.")
        print("Use --dry-run to see what would change without making modifications.")
        sys.exit(1)
    
    repo_path = Path(sys.argv[1]).resolve()
    dry_run = '--dry-run' in sys.argv
    
    if not repo_path.exists():
        print(f"Error: Path does not exist: {repo_path}")
        sys.exit(1)
    
    if not repo_path.is_dir():
        print(f"Error: Path is not a directory: {repo_path}")
        sys.exit(1)
    
    print("="*60)
    print("JUNITA REBRANDING SCRIPT")
    print("="*60)
    print(f"Repository: {repo_path}")
    print(f"Mode: {'DRY RUN (no changes will be made)' if dry_run else 'LIVE (files will be modified)'}")
    print("="*60)
    
    if not dry_run:
        response = input("\nThis will modify files in your repository. Continue? [y/N]: ")
        if response.lower() != 'y':
            print("Aborted.")
            sys.exit(0)
    
    print("\nScanning repository...")
    files_to_process = find_files_to_process(repo_path)
    print(f"Found {len(files_to_process)} files to process")
    
    print("\nProcessing files...")
    results = []
    for i, filepath in enumerate(files_to_process, 1):
        if i % 10 == 0:
            print(f"  Progress: {i}/{len(files_to_process)}", end='\r')
        
        result = rebrand_file(filepath, dry_run=dry_run)
        results.append(result)
    
    print(f"  Progress: {len(files_to_process)}/{len(files_to_process)} - Done!")
    
    print("\nRenaming files and directories...")
    renames = rename_paths(repo_path, dry_run=dry_run)
    
    print_summary(results, renames)
    
    if dry_run:
        print("\n⚠️  This was a DRY RUN. No files were modified.")
        print("Run without --dry-run to apply changes.")
    else:
        print("\n✅ Rebranding complete!")
        print("\nNext steps:")
        print("1. Review the changes: git diff")
        print("2. Test that everything still builds/runs")
        print("3. Update your logo files")
        print("4. Commit: git add -A && git commit -m 'Rebrand to Junita'")

if __name__ == '__main__':
    main()

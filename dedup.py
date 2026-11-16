from argparse import ArgumentParser
from collections import defaultdict
from dataclasses import dataclass
from hashlib import sha256
from pathlib import Path
from time import time

type GroupsBySize = defaultdict[int, list[Path]]
type GroupsByHash = defaultdict[str, list[Path]]


def locate_image_files(directory: Path) -> list[Path]:
    allowed_extensions = ("png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp")

    files = []

    for item in directory.iterdir():
        if item.is_file():
            file_extension = item.suffix.lstrip(".")

            if file_extension.lower() in allowed_extensions:
                files.append(item)

    return files


def group_files_by_size(files: list[Path]) -> GroupsBySize:
    groups = defaultdict(list)

    for filepath in files:
        groups[filepath.stat().st_size].append(filepath)

    return groups


def get_sha256_file_hash(filepath: Path, chunk_size: int = 8192) -> str:
    hasher = sha256()

    with filepath.open("rb") as f:
        while chunk := f.read(chunk_size):
            hasher.update(chunk)

    return hasher.hexdigest()


def group_files_by_hash(groups_by_size: GroupsBySize) -> GroupsByHash:
    groups = defaultdict(list)

    for paths in groups_by_size.values():
        if len(paths) < 2:
            continue

        for filepath in paths:
            file_hash = get_sha256_file_hash(filepath)
            groups[file_hash].append(filepath)

    return groups


def isolate_duplicates(groups_by_hash: GroupsByHash) -> list[Duplicates]:
    duplicates = []

    for file_hash, paths in groups_by_hash.items():
        if len(paths) > 1:
            duplicates.append(Duplicates(file_hash=file_hash, file_paths=paths))

    return duplicates


@dataclass
class Duplicates:
    file_hash: str
    file_paths: list[Path]

    def __len__(self) -> int:
        return len(self.file_paths)

    def delete_all_except(self, idx_to_keep: int) -> None:
        for idx, filepath in enumerate(self.file_paths):
            if idx == idx_to_keep:
                print(f" (+) {filepath}")
            else:
                filepath.unlink()
                print(f" (-) {filepath}")


def delete_duplicate_files(duplicates: Duplicates) -> None:
    for u, filepath in enumerate(duplicates.file_paths):
        print(f" -> [{u}] keep this file: {filepath}")

    while True:
        try:
            idx_to_keep = int(input(f"input an option [0 to {len(duplicates) - 1}]: "))
        except ValueError:
            print(f"enter a valid integer between 0 and {len(duplicates) - 1}")

        if 0 <= idx_to_keep <= (len(duplicates) - 1):
            break

        print(f"enter a valid integer between 0 and {len(duplicates) - 1}")

    duplicates.delete_all_except(idx_to_keep)


def loop_over_duplicates(duplicates: list[Duplicates]) -> None:
    print("*** press Ctrl-C to exit ***\n")

    for u, duplicate_files in enumerate(duplicates, 1):
        print(f"({u}) found duplicates with hash: {duplicate_files.file_hash}")
        delete_duplicate_files(duplicate_files)


def print_duplicate_files(duplicates: list[Duplicates]) -> None:
    for u, duplicate_files in enumerate(duplicates, 1):
        print(f"({u}) found duplicates with hash: {duplicate_files.file_hash}")
        for path in duplicate_files.file_paths:
            print(f" -> {path}")


def process_directory(target_dir: Path, dry_run: bool) -> None:
    if not target_dir.exists():
        print(f"directory '{target_dir}' does not exist")
        return

    start_time = time()
    image_files = locate_image_files(target_dir)
    files_grouped_by_size = group_files_by_size(image_files)
    files_grouped_by_hash = group_files_by_hash(files_grouped_by_size)
    duplicates = isolate_duplicates(files_grouped_by_hash)
    search_time = round(time() - start_time, 3)

    print(f"found {len(image_files)} image files")
    print(f"found {len(files_grouped_by_size)} size-identical groups")
    print(f"found {len(files_grouped_by_hash)} hash-identical groups")
    print(f"total search time: {search_time}s\n")

    if len(duplicates) == 0:
        print("no duplicate files found")
        return

    if dry_run:
        print_duplicate_files(duplicates)
    else:
        loop_over_duplicates(duplicates)


def main() -> None:
    parser = ArgumentParser(description="Deduplicates image files in a directory")
    parser.add_argument(
        "path",
        nargs="?",
        default=".",
        help="path to search for duplicates (defaults to current working directory)",
    )
    parser.add_argument(
        "-d",
        "--dry-run",
        action="store_true",
        help="preview duplicates without deleting them",
    )

    args = parser.parse_args()

    try:
        process_directory(target_dir=Path(args.path), dry_run=args.dry_run)
    except KeyboardInterrupt:
        print("\nprocess was manually aborted!")
    except OSError as e:
        raise SystemExit(f"an error occurred: {e}") from e


if __name__ == "__main__":
    main()

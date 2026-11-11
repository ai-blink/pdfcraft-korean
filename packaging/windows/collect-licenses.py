"""Stage full source and dependency notices for a binary distribution; never copy code."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tomllib
import re
import urllib.error
import urllib.request


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--destination", type=Path, required=True)
    parser.add_argument("--target", default="x86_64-pc-windows-msvc")
    args = parser.parse_args()
    repo, destination = args.repo.resolve(), args.destination.resolve()
    if destination.exists():
        raise ValueError("Use a fresh notice directory; existing evidence is not overwritten")
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--locked", "--format-version", "1", "--filter-platform", args.target], cwd=repo))
    pending, inventory = [], []
    downloaded = {}
    for package in metadata["packages"]:
        root = Path(package["manifest_path"]).parent
        candidates = set()
        if package.get("license_file"):
            candidates.add((root / package["license_file"]).resolve())
        # Workspace libraries may inherit the license kept at the checkout root.
        for directory in [root, *list(root.parents)[:3]]:
            found = [p for p in directory.iterdir() if p.is_file()
                     and p.name.upper().startswith(("LICENSE", "LICENCE", "COPYING", "COPYRIGHT", "NOTICE"))
                     and p.suffix.lower() not in (".toml", ".json", ".rs")]
            candidates.update(found)
            if candidates:
                break
        choice = None
        expression = (package.get("license") or "").replace("/", " OR ")
        if not candidates and "Apache-2.0" in expression.split() and "AND" not in expression.split() and "WITH" not in expression.split():
            # An explicit OR grant permits Apache-2.0. Include its complete standard text,
            # package authors/manifest grant and original README; never invent a MIT copyright.
            candidates.add(repo / "LICENSE-APACHE")
            if (root / "README.md").is_file():
                candidates.add(root / "README.md")
            choice = "Apache-2.0"
        remote = []
        if not candidates:
            vcs = root / ".cargo_vcs_info.json"
            repository = re.match(r"https://github.com/([\w.-]+/[\w.-]+)", package.get("repository") or "")
            revision = json.loads(vcs.read_text())["git"]["sha1"] if vcs.exists() else ""
            if not repository or not re.fullmatch(r"[0-9a-f]{40}", revision):
                raise ValueError(f"No pinned license source for {package['name']} {package['version']}")
            paths = [""]
            subdir = json.loads(vcs.read_text()).get("path_in_vcs", "").strip("/")
            if subdir:
                paths.append(subdir + "/")
            for filename in [prefix + name for prefix in paths for name in ["LICENSE-MIT", "LICENSE-APACHE", "LICENSE", "COPYING"]]:
                url = f"https://raw.githubusercontent.com/{repository[1].removesuffix(chr(46)+"git")}/{revision}/{filename}"
                if url not in downloaded:
                    try:
                        with urllib.request.urlopen(url, timeout=30) as response:
                            downloaded[url] = response.read(2_000_001)
                    except urllib.error.HTTPError as error:
                        if error.code != 404:
                            raise
                        downloaded[url] = None
                if downloaded[url]:
                    remote.append((Path(filename).name, downloaded[url], url))
            if not remote:
                raise ValueError(f"No license text at the pinned source for {package['name']}")
        entries = []
        for source in sorted(candidates):
            content = source.read_bytes()
            content.decode("utf-8")  # refuse accidental binary payloads
            if not content or len(content) > 2_000_000:
                raise ValueError(f"Invalid license text for {package['name']}")
            relative = Path("cargo") / (package["name"] + "-" + package["version"]) / source.name
            pending.append((relative, content))
            entries.append({"path": relative.as_posix(), "sha256": hashlib.sha256(content).hexdigest()})
        for filename, content, url in remote:
            content.decode("utf-8")
            if len(content) > 2_000_000:
                raise ValueError("Remote license too large")
            relative = Path("cargo") / (package["name"] + "-" + package["version"]) / filename
            pending.append((relative, content))
            entries.append({"path": relative.as_posix(), "sha256": hashlib.sha256(content).hexdigest(), "source": url})
        inventory.append({"package": package["name"], "version": package["version"],
                          "license": package.get("license"), "license_choice": choice, "authors": package.get("authors", []),
                          "repository": package.get("repository"), "documents": entries})
    manifest = tomllib.loads((repo / "LICENSES.toml").read_text(encoding="utf-8"))
    for document in manifest["document"]:
        source = (repo / document["source"]).resolve()
        if not source.is_relative_to(repo):
            raise ValueError("Source license escaped the repository")
        content = source.read_bytes()
        if hashlib.sha256(content).hexdigest() != document["sha256"]:
            raise ValueError(f"License hash mismatch: {document['source']}")
        pending.append((Path("source") / document["destination"], content))
    for relative, content in pending:
        target = destination / relative
        if not target.resolve().is_relative_to(destination):
            raise ValueError("Notice destination escaped its directory")
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(content)
    (destination / "inventory.json").write_text(json.dumps(inventory, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"Staged {len(pending)} full notice files for {len(inventory)} Cargo packages")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Offline documentation gate; jsonschema validates schemas, never vendor APIs."""
import json
import re
from pathlib import Path
from urllib.parse import unquote, urlsplit

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
IGNORED = {".git", ".research", ".venv", "__pycache__"}


def files(pattern):
    return sorted(p for p in ROOT.rglob(pattern) if not IGNORED.intersection(p.relative_to(ROOT).parts))


def load(path):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            assert key not in result, f"Duplicate JSON key: {key}"
            result[key] = value
        return result

    def invalid_constant(value):
        raise ValueError(f"Non-JSON numeric constant: {value}")

    return json.loads(path.read_text(), object_pairs_hook=unique, parse_constant=invalid_constant)


def resolve_link(base, target):
    parsed = urlsplit(target)
    if parsed.scheme or parsed.netloc:
        return
    path = (base.parent / unquote(parsed.path)).resolve() if parsed.path else base
    assert path.is_relative_to(ROOT), f"Link escapes repository: {target}"
    assert path.exists(), f"Broken link in {base.relative_to(ROOT)}: {target}"
    fragment = unquote(parsed.fragment)
    if not fragment:
        return
    if path.suffix == ".json":
        node = load(path)
        assert fragment.startswith("/"), f"Expected JSON Pointer: {target}"
        for part in fragment[1:].split("/"):
            key = part.replace("~1", "/").replace("~0", "~")
            node = node[int(key)] if isinstance(node, list) else node[key]
    elif path.suffix == ".md":
        headings = re.findall(r"^#{1,6}\s+(.+)$", path.read_text(), re.M)
        anchors = {re.sub(r"[^\w\- ]", "", h.lower()).replace(" ", "-") for h in headings}
        assert fragment in anchors, f"Broken heading in {base.relative_to(ROOT)}: {target}"


def main():
    json_files = files("*.json")
    for path in json_files:
        load(path)
    schemas = files("*.schema.json")
    for path in schemas:
        schema = load(path)
        Draft202012Validator.check_schema(schema)
        # No network references are needed or permitted in this offline gate.
        assert '"$ref"' not in json.dumps(schema), f"Unexpected reference in {path.name}"

    manifest = load(ROOT / "examples/manifest.json")
    checked = set()
    valid_count = invalid_count = 0
    for fixture in manifest["fixtures"]:
        path = ROOT / fixture["file"]
        assert path not in checked, f"Duplicate fixture declaration: {fixture['file']}"
        checked.add(path)
        schema = load(ROOT / fixture["schema"])
        errors = list(Draft202012Validator(schema).iter_errors(load(path)))
        if fixture["valid"]:
            assert not errors, f"{fixture['file']}: " + "; ".join(e.message for e in errors[:3])
            valid_count += 1
        else:
            assert errors, f"Invalid fixture unexpectedly accepted: {fixture['file']}"
            invalid_count += 1
    expected = set((ROOT / "examples").rglob("*.json")) - {ROOT / "examples/manifest.json"}
    assert checked == expected, "Every example must declare a schema and validity expectation"

    fenced = links = 0
    for path in files("*.md"):
        text = path.read_text()
        for block in re.findall(r"^```json\s*\n(.*?)^```\s*$", text, re.M | re.S):
            json.loads(block)
            fenced += 1
        # Inline Markdown links/images used by this repository; code fences excluded.
        prose = re.sub(r"^```[^\n]*\n.*?^```\s*$", "", text, flags=re.M | re.S)
        for target in re.findall(r"!?\[[^\]\n]*\]\(([^)\s]+)\)", prose):
            resolve_link(path, target)
            links += 1

    coverage = load(ROOT / "coverage.json")
    Draft202012Validator(load(ROOT / "schemas/coverage.schema.json")).validate(coverage)
    operations = set()
    entries = response_paths = 0
    for endpoint in coverage["endpoints"]:
        assert endpoint["operation"] not in operations
        operations.add(endpoint["operation"])
        for key in ["document", "provider_schema", "response_schema"]:
            resolve_link(ROOT / "coverage.json", endpoint[key])
        seen = set()
        for field in endpoint["request_fields"]:
            key = (field["location"], field["name"], tuple(field["methods"]))
            assert key not in seen, f"Duplicate inventory entry: {key}"
            seen.add(key)
            resolve_link(ROOT / "coverage.json", field["schema"])
            resolve_link(ROOT / "coverage.json", field["document"])
            entries += 1
        # Every decoded upstream schema field must have a location-specific entry.
        request = load(ROOT / f"schemas/upstream/{endpoint['operation']}.request.schema.json")
        for section, location in [("query", "query"), ("headers", "header")]:
            for name in request["properties"][section]["properties"]:
                assert any(f["name"] == name and f["location"] == location for f in endpoint["request_fields"]), name
        catalog = ROOT / f"docs/api/{endpoint['operation']}-response.md"
        paths = re.findall(r"^\| `([^`]+)` \| `[^`]+` \|", catalog.read_text(), re.M)
        assert len(paths) == endpoint["response_reference_paths"], f"Response catalog count: {catalog.name}"
        for field in paths:
            pointer = ""
            for part in field.split("."):
                name = part.split("[")[0]
                pointer += "/properties/" + name
                pointer += "/items" * part.count("[]")
            # The array field itself names the array, not its items.
            pointer = pointer.removesuffix("/items" * field.split(".")[-1].count("[]")) if field.endswith("[]") else pointer
            resolve_link(ROOT / "coverage.json", endpoint["response_schema"] + "#" + pointer)
        response_paths += len(paths)
    assert operations == {"web", "context", "news", "images", "videos", "place", "local-pois", "poi-descriptions", "rich", "suggest", "spellcheck"}
    print(f"PASS: {len(json_files)} JSON files; {len(schemas)} well-formed schemas; {valid_count} valid and {invalid_count} invalid fixtures")
    print(f"PASS: {fenced} JSON fences; {links} Markdown links; coverage/schema JSON Pointers")
    print(f"PASS: {len(operations)} operations; {entries} request-field/location entries; {response_paths} reference response paths")
    print("NOT PROVEN: live API compatibility, exhaustive vendor validation, legal entitlement, auth integration, runtime byte limits")


if __name__ == "__main__":
    main()

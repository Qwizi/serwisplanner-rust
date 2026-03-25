#!/usr/bin/env python3
"""
Generate Rust types and resource modules from the AURA API OpenAPI spec.

Generates one file per resource module under src/generated/<mod>.rs,
plus src/generated/mod.rs and updates to src/client.rs accessors.

Usage:
    python3 codegen.py /path/to/aura-api-full.yaml
"""
import sys
import re
import shutil
import yaml
from pathlib import Path
from collections import defaultdict

SRC = Path(__file__).parent / "src"
GEN_DIR = SRC / "generated"

RUST_RESERVED = {
    "type", "self", "super", "crate", "mod", "fn", "let", "mut",
    "ref", "match", "if", "else", "for", "while", "loop", "return",
    "struct", "enum", "use", "pub", "static", "const", "where",
    "async", "await", "move", "in", "as", "impl", "trait", "dyn",
}

# Global: all schemas (set in main)
ALL_SCHEMAS: dict = {}
# Track which relation structs we need to generate per module
RELATION_STRUCTS: dict = {}  # {module_resource: [(struct_name, schema_name)]}


def snake_case(name: str) -> str:
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    return s.lower()


def ref_to_schema_name(ref: str) -> str:
    """Extract schema name from $ref like '#/components/schemas/FooBar'"""
    return ref.rsplit("/", 1)[-1]


def ref_to_struct_name(ref: str) -> str:
    """Convert $ref to a Rust struct name."""
    schema_name = ref_to_schema_name(ref)
    # e.g. AccountUserGETFieldsForRelation -> AccountUserRelation
    name = schema_name.replace("GETFieldsForRelation", "Rel")
    name = name.replace("GETFields", "")
    return name


def resolve_ref_props(ref: str) -> dict:
    """Resolve $ref to its properties."""
    schema_name = ref_to_schema_name(ref)
    schema = ALL_SCHEMAS.get(schema_name, {})
    return get_schema_props(schema)


def to_rust_type(prop: dict, module_resource: str | None = None) -> str:
    """Map OpenAPI property to Rust type. Tracks relation structs needed."""
    ptype = prop.get("type", "")
    ref_ = prop.get("$ref", "")
    items = prop.get("items", {})
    all_of = prop.get("allOf", [])

    # Direct $ref
    if ref_:
        rel_props = resolve_ref_props(ref_)
        if not rel_props:
            return "serde_json::Value"
        struct_name = ref_to_struct_name(ref_)
        if module_resource:
            RELATION_STRUCTS.setdefault(module_resource, set()).add(
                (struct_name, ref_to_schema_name(ref_))
            )
        return struct_name

    # allOf with $ref (relation fields)
    if all_of:
        for item in all_of:
            if isinstance(item, dict) and "$ref" in item:
                return to_rust_type(item, module_resource)
        return "serde_json::Value"

    if ptype == "integer":
        return "i64"
    elif ptype == "number":
        return "f64"
    elif ptype == "boolean":
        return "bool"
    elif ptype == "string":
        return "String"
    elif ptype == "array":
        if items:
            # Handle anyOf in items (attribute types etc.)
            if "anyOf" in items or "oneOf" in items:
                return "Vec<serde_json::Value>"
            inner = to_rust_type(items, module_resource)
            return f"Vec<{inner}>"
        return "Vec<serde_json::Value>"
    elif ptype == "object":
        return "serde_json::Value"
    else:
        return "serde_json::Value"


def get_schema_props(schema: dict) -> dict:
    props = schema.get("properties", {})
    if not props and "allOf" in schema:
        for item in schema.get("allOf", []):
            if isinstance(item, dict) and "properties" in item:
                props.update(item["properties"])
    return props


def generate_struct(name: str, props: dict, module_resource: str | None = None) -> str:
    lines = [
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        "#[serde(default, rename_all = \"camelCase\")]",
        f"pub struct {name} {{",
    ]
    for pname, pdef in sorted(props.items()):
        rust_type = to_rust_type(pdef, module_resource)
        rust_field = snake_case(pname)

        if rust_field in RUST_RESERVED:
            lines.append(f"    #[serde(rename = \"{pname}\")]")
            rust_field = f"r#{rust_field}"

        if not rust_type.startswith("Option<"):
            rust_type = f"Option<{rust_type}>"
        lines.append(f"    pub {rust_field}: {rust_type},")

    lines.append("}")
    return "\n".join(lines)


def pluralize(word: str) -> list[str]:
    forms = [word + "s"]
    if word.endswith("y"):
        forms.append(word[:-1] + "ies")
        forms.append(word[:-1] + "aies")
    if word.endswith("s") or word.endswith("x") or word.endswith("sh"):
        forms.append(word + "es")
    forms.append(word)
    return forms


def find_api_path(resource: str, api_paths: dict) -> str | None:
    candidates = []
    for p in api_paths:
        segments = p.strip("/").split("/")
        if len(segments) >= 2:
            base = segments[1].split("{")[0].rstrip("/")
            candidates.append((p, base))

    sn = snake_case(resource)
    for tag_var in pluralize(sn):
        for p, base in candidates:
            if base == tag_var:
                return f"/api/{base}"
        test = f"/api/{tag_var}"
        if test in api_paths or f"{test}/{{id}}" in api_paths:
            return test
    return None


def generate_module_file(
    resource: str,
    props: dict,
    api_path: str | None,
) -> str:
    mod_name = snake_case(resource)
    RELATION_STRUCTS.pop(resource, None)  # Reset for this module

    lines = [
        f"//! Auto-generated module for `{resource}`.",
        "//!",
        "//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`",
        "",
    ]

    if api_path:
        lines.extend([
            "use std::sync::Arc;",
            "use serde_json::Value;",
            "use crate::client::ClientInner;",
            "use crate::error::Result;",
            "use crate::params::QueryParams;",
            "use crate::resources::base::Resource;",
            "",
        ])

    # Main struct
    lines.append(generate_struct(resource, props, resource))
    lines.append("")

    # Generate relation structs needed by this module
    if resource in RELATION_STRUCTS:
        for struct_name, schema_name in sorted(RELATION_STRUCTS[resource]):
            rel_props = resolve_ref_props(f"#/components/schemas/{schema_name}")
            if rel_props:
                # Don't recurse into relations of relations (use Value)
                lines.append(generate_struct(struct_name, rel_props, None))
                lines.append("")

    # ListResponse
    lines.append(f"""#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct {resource}ListResponse {{
    pub data: Vec<{resource}>,
    pub meta: Option<super::ListMeta>,
}}""")
    lines.append("")

    # Resource accessor
    if api_path:
        lines.append(f"""/// Resource accessor for `{api_path}`.
pub struct {resource}Resource {{
    resource: Resource,
}}

impl {resource}Resource {{
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {{
        Self {{
            resource: Resource::new(inner, "{api_path}"),
        }}
    }}

    /// List resources.
    pub async fn list(&self, params: Option<&QueryParams>) -> Result<{resource}ListResponse> {{
        let value = self.resource.list(params).await?;
        serde_json::from_value(value).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }}

    /// Retrieve a single resource by ID.
    pub async fn retrieve(&self, id: u64, params: Option<&QueryParams>) -> Result<{resource}> {{
        let value = self.resource.retrieve(id, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }}

    /// Create a new resource.
    pub async fn create(&self, data: &Value, params: Option<&QueryParams>) -> Result<{resource}> {{
        let value = self.resource.create(data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }}

    /// Update a resource.
    pub async fn update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<{resource}> {{
        let value = self.resource.update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }}

    /// Partial update a resource.
    pub async fn partial_update(&self, id: u64, data: &Value, params: Option<&QueryParams>) -> Result<{resource}> {{
        let value = self.resource.partial_update(id, data, params).await?;
        let inner = value.get("data").cloned().unwrap_or(value);
        serde_json::from_value(inner).map_err(|e| crate::error::SWError::Other(e.to_string()))
    }}

    /// Delete a resource.
    pub async fn delete(&self, id: u64, params: Option<&QueryParams>) -> Result<Value> {{
        self.resource.delete(id, params).await
    }}

    /// Fetch all pages automatically.
    pub async fn all(&self, params: Option<QueryParams>) -> Result<Vec<{resource}>> {{
        let items = self.resource.all(params).await?;
        items.into_iter()
            .map(|v| serde_json::from_value(v).map_err(|e| crate::error::SWError::Other(e.to_string())))
            .collect()
    }}

    /// Get resource metadata.
    pub async fn meta(&self, params: Option<&QueryParams>) -> Result<Value> {{
        self.resource.meta(params).await
    }}

    /// Autoselect.
    pub async fn autoselect(&self, params: Option<&QueryParams>) -> Result<Value> {{
        self.resource.autoselect(params).await
    }}

    /// Access the untyped base resource.
    pub fn raw(&self) -> &Resource {{
        &self.resource
    }}
}}""")

    return "\n".join(lines) + "\n"


def main():
    global ALL_SCHEMAS

    if len(sys.argv) < 2:
        print("Usage: codegen.py <path-to-yaml>")
        sys.exit(1)

    with open(sys.argv[1]) as f:
        spec = yaml.safe_load(f)

    ALL_SCHEMAS = spec.get("components", {}).get("schemas", {})
    api_paths = spec.get("paths", {})

    # Deprecated
    deprecated_tags = set()
    for path, methods in api_paths.items():
        for method, details in methods.items():
            if isinstance(details, dict) and details.get("deprecated"):
                for tag in details.get("tags", []):
                    deprecated_tags.add(tag)

    # Active resources
    all_resources = {}
    for schema_name, schema in ALL_SCHEMAS.items():
        if not schema_name.endswith("GETFields"):
            continue
        if "ForCollection" in schema_name or "ForRelation" in schema_name:
            continue
        resource = schema_name.replace("GETFields", "")
        if resource in deprecated_tags:
            continue
        props = get_schema_props(schema)
        if props:
            all_resources[resource] = props

    # Clean generated dir
    if GEN_DIR.exists():
        shutil.rmtree(GEN_DIR)
    GEN_DIR.mkdir(parents=True)

    # Generate per-module files
    mod_names = []
    for resource in sorted(all_resources.keys()):
        props = all_resources[resource]
        api_path = find_api_path(resource, api_paths)
        has_resource = api_path is not None

        mod_name = snake_case(resource)
        content = generate_module_file(resource, props, api_path)

        file_path = GEN_DIR / f"{mod_name}.rs"
        file_path.write_text(content)
        mod_names.append((mod_name, resource, has_resource, api_path))

    # Generate mod.rs
    mod_lines = [
        "//! Auto-generated modules from AURA API OpenAPI specification.",
        "//!",
        "//! Do not edit manually. Regenerate with: `python3 codegen.py <yaml>`",
        "",
        "/// Pagination metadata returned in list responses.",
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        "#[serde(default)]",
        "pub struct ListMeta {",
        "    pub total: Option<i64>,",
        "    pub limit: Option<i64>,",
        "    pub offset: Option<i64>,",
        "}",
        "",
    ]

    for mod_name, resource, has_resource, _ in mod_names:
        mod_lines.append(f"pub mod {mod_name};")

    mod_lines.append("")

    for mod_name, resource, has_resource, _ in mod_names:
        mod_lines.append(f"pub use {mod_name}::{{{resource}, {resource}ListResponse}};")
        if has_resource:
            mod_lines.append(f"pub use {mod_name}::{resource}Resource;")

    mod_path = GEN_DIR / "mod.rs"
    mod_path.write_text("\n".join(mod_lines) + "\n")

    # Generate _generated_accessors.rs
    acc_lines = [
        "// Auto-generated resource accessor methods.",
        "// Do not edit manually. Regenerate with: python3 codegen.py <yaml>",
        "",
        "impl SerwisPlanner {",
    ]
    accessor_count = 0
    for mod_name, resource, has_resource, api_path in mod_names:
        if not has_resource:
            continue
        acc_lines.append(f"    /// `{api_path}`")
        acc_lines.append(f"    pub fn {mod_name}(&self) -> crate::generated::{resource}Resource {{")
        acc_lines.append(f"        crate::generated::{resource}Resource::new(self.inner.clone())")
        acc_lines.append(f"    }}")
        acc_lines.append("")
        accessor_count += 1
    acc_lines.append("}")

    acc_path = SRC / "_generated_accessors.rs"
    acc_path.write_text("\n".join(acc_lines) + "\n")

    # Stats
    active_count = len(all_resources)
    rel_count = sum(len(v) for v in RELATION_STRUCTS.values())
    print(f"Generated {len(mod_names)} modules in {GEN_DIR}/")
    print(f"  {active_count} types + {rel_count} relation types")
    print(f"  {accessor_count} resource accessors")
    print(f"Generated {acc_path}")


if __name__ == "__main__":
    main()

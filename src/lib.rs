// SPDX-License-Identifier: MPL-2.0
//! Search-only adaptation of Brave's `bx` interface at commit
//! 5df7479f457e176baa250b2abad944f19628d0b8 (MPL-2.0).
//! The CLI vocabulary and endpoint/field mapping are adapted from src/main.rs;
//! native networking, ambient configuration and credential handling are not copied.

use dekopon_provider_http::{Header, HttpError, Request, Response, method};
use dekopon_provider_sdk::clap::{self, Arg, ArgAction, ArgMatches, Command};
use dekopon_provider_sdk::{
    CapabilityId, CommandInvocation, CommandRun, EffectKind, Provider, ProviderApiVersion,
    ProviderCapability, ProviderError, ProviderManifest, RiskLevel, cli,
};
use serde_json::{Map, Value, json};

mod bindings {
    wit_bindgen::generate!({ path: "wit", world: "provider", generate_all, pub_export_macro: true });
}

struct Brave;
const ORIGIN: &str = "https://api.search.brave.com";
// The broker owns authorization. In particular the guest cannot supply X-Subscription-Token.
const MAX_TEXT: usize = 4096;
const MAX_QUERY: usize = 2048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Text,
    Number,
    Boolean,
    List,
    Choice(&'static [&'static str]),
}
#[derive(Clone, Copy)]
struct Field {
    name: &'static str,
    kind: Kind,
}
const fn t(name: &'static str) -> Field {
    Field {
        name,
        kind: Kind::Text,
    }
}
const fn n(name: &'static str) -> Field {
    Field {
        name,
        kind: Kind::Number,
    }
}
const fn b(name: &'static str) -> Field {
    Field {
        name,
        kind: Kind::Boolean,
    }
}
const fn l(name: &'static str) -> Field {
    Field {
        name,
        kind: Kind::List,
    }
}
const fn c(name: &'static str, choices: &'static [&'static str]) -> Field {
    Field {
        name,
        kind: Kind::Choice(choices),
    }
}
const SAFE: &[&str] = &["off", "moderate", "strict"];
const IMAGE_SAFE: &[&str] = &["off", "strict"];
const UNITS: &[&str] = &["metric", "imperial"];
const LOCATION: &[Field] = &[
    t("lat"),
    t("long"),
    t("timezone"),
    t("city"),
    t("state"),
    t("state_name"),
    t("loc_country"),
    t("postal_code"),
];
const GOGGLES: &[Field] = &[l("goggles"), l("include_site"), l("exclude_site")];
const WEB: &[Field] = &[
    t("country"),
    t("search_lang"),
    t("ui_lang"),
    n("count"),
    n("offset"),
    c("safesearch", SAFE),
    t("freshness"),
    b("text_decorations"),
    b("spellcheck"),
    l("result_filter"),
    b("extra_snippets"),
    c("units", UNITS),
    b("operators"),
];
const NEWS: &[Field] = &[
    t("country"),
    t("search_lang"),
    t("ui_lang"),
    n("count"),
    n("offset"),
    c("safesearch", SAFE),
    t("freshness"),
    b("spellcheck"),
    b("extra_snippets"),
    b("operators"),
];
const CONTEXT: &[Field] = &[
    t("country"),
    t("search_lang"),
    n("count"),
    n("maximum_number_of_urls"),
    n("maximum_number_of_tokens"),
    n("maximum_number_of_snippets"),
    n("maximum_number_of_tokens_per_url"),
    n("maximum_number_of_snippets_per_url"),
    c(
        "context_threshold_mode",
        &["strict", "balanced", "lenient", "disabled"],
    ),
    b("enable_local"),
];
const IMAGES: &[Field] = &[
    t("country"),
    t("search_lang"),
    n("count"),
    c("safesearch", IMAGE_SAFE),
    b("spellcheck"),
];
const VIDEOS: &[Field] = &[
    t("country"),
    t("search_lang"),
    t("ui_lang"),
    n("count"),
    n("offset"),
    c("safesearch", SAFE),
    t("freshness"),
    b("spellcheck"),
    b("operators"),
];
const PLACES: &[Field] = &[
    t("latitude"),
    t("longitude"),
    t("location"),
    n("radius"),
    n("count"),
    t("country"),
    t("search_lang"),
    t("ui_lang"),
    c("units", UNITS),
    c("safesearch", SAFE),
    b("spellcheck"),
];
const POIS: &[Field] = &[
    t("search_lang"),
    t("ui_lang"),
    c("units", UNITS),
    t("lat"),
    t("long"),
];
const SUGGEST: &[Field] = &[t("lang"), t("country"), n("count"), b("rich")];
const SPELLCHECK: &[Field] = &[t("lang"), t("country")];
const ANSWERS: &[Field] = &[
    c("model", &["brave-pro", "brave"]),
    t("country"),
    t("language"),
    c("safesearch", SAFE),
    n("max_completion_tokens"),
    c("search_context_size", &["low", "medium", "high"]),
    t("user_city"),
    t("user_country"),
    t("user_region"),
    t("user_timezone"),
];

#[derive(Clone, Copy)]
struct Operation {
    word: &'static str,
    path: &'static str,
    post: bool,
    query: bool,
    fields: &'static [Field],
    goggles: bool,
    location: bool,
}
const OPS: &[Operation] = &[
    Operation {
        word: "context",
        path: "/res/v1/llm/context",
        post: true,
        query: true,
        fields: CONTEXT,
        goggles: true,
        location: true,
    },
    Operation {
        word: "web",
        path: "/res/v1/web/search",
        post: true,
        query: true,
        fields: WEB,
        goggles: true,
        location: true,
    },
    Operation {
        word: "news",
        path: "/res/v1/news/search",
        post: true,
        query: true,
        fields: NEWS,
        goggles: true,
        location: false,
    },
    Operation {
        word: "images",
        path: "/res/v1/images/search",
        post: false,
        query: true,
        fields: IMAGES,
        goggles: false,
        location: false,
    },
    Operation {
        word: "videos",
        path: "/res/v1/videos/search",
        post: true,
        query: true,
        fields: VIDEOS,
        goggles: false,
        location: false,
    },
    Operation {
        word: "places",
        path: "/res/v1/local/place_search",
        post: false,
        query: false,
        fields: PLACES,
        goggles: false,
        location: false,
    },
    Operation {
        word: "pois",
        path: "/res/v1/local/pois",
        post: false,
        query: false,
        fields: POIS,
        goggles: false,
        location: false,
    },
    Operation {
        word: "descriptions",
        path: "/res/v1/local/descriptions",
        post: false,
        query: false,
        fields: &[],
        goggles: false,
        location: false,
    },
    Operation {
        word: "suggest",
        path: "/res/v1/suggest/search",
        post: false,
        query: true,
        fields: SUGGEST,
        goggles: false,
        location: false,
    },
    Operation {
        word: "spellcheck",
        path: "/res/v1/spellcheck/search",
        post: false,
        query: true,
        fields: SPELLCHECK,
        goggles: false,
        location: false,
    },
    Operation {
        word: "answers",
        path: "/res/v1/chat/completions",
        post: true,
        query: true,
        fields: ANSWERS,
        goggles: false,
        location: false,
    },
];

fn fields(op: Operation) -> impl Iterator<Item = Field> {
    op.fields
        .iter()
        .chain(if op.goggles { GOGGLES } else { &[] })
        .chain(if op.location { LOCATION } else { &[] })
        .copied()
}
fn id(op: Operation) -> String {
    format!("bx.{}", op.word)
}
fn usage(msg: impl Into<String>) -> ProviderError {
    ProviderError::new("usage", msg)
}

// Limits in Brave's search endpoint references; keep model-facing schemas and native
// validation on the same bounds. Fields without a published upper limit retain the
// existing request ceiling, rather than borrowing another operation's count limit.
fn number_bounds(op: Operation, field: &str) -> (u64, u64) {
    match (op.word, field) {
        ("web" | "suggest", "count") => (1, 20),
        ("context" | "news" | "videos", "count") => (1, 50),
        ("images", "count") => (1, 200),
        ("places", "count") => (1, 100),
        ("web" | "news" | "videos", "offset") => (0, 9),
        ("context", "maximum_number_of_urls") => (1, 50),
        ("context", "maximum_number_of_tokens") => (1024, 32768),
        ("context", "maximum_number_of_snippets") => (1, 256),
        ("context", "maximum_number_of_tokens_per_url") => (512, 8192),
        ("context", "maximum_number_of_snippets_per_url") => (1, 100),
        _ => (0, 100_000),
    }
}

impl Provider for Brave {
    fn manifest() -> ProviderManifest {
        ProviderManifest { api_version: ProviderApiVersion::V1Alpha1,
            id: "bx".parse().expect("static ID"), description: "Brave search-family operations via broker HTTP (broker X-Subscription-Token support required)".into(),
            command_words: vec!["bx".into()],
            capabilities: OPS.iter().copied().map(|op| {
                let mut properties = Map::new();
                if op.query || op.word == "places" { properties.insert("q".into(), json!({"type":"string", "minLength":1, "maxLength":MAX_QUERY})); }
                if matches!(op.word, "pois" | "descriptions") { properties.insert("ids".into(), json!({"type":"array", "items":{"type":"string"}, "minItems":1, "maxItems":20})); }
                for field in fields(op) {
                    let schema = match field.kind { Kind::Text => json!({"type":"string"}), Kind::Number => { let (min, max) = number_bounds(op, field.name); json!({"type":"integer", "minimum":min, "maximum":max}) }, Kind::Boolean => json!({"type":"boolean"}), Kind::List => if field.name == "goggles" { json!({"type":"array", "items":{"type":"string"}, "minItems":1, "maxItems":3}) } else { json!({"type":"array", "items":{"type":"string"}}) }, Kind::Choice(choices) => json!({"type":"string", "enum":choices}) };
                    properties.insert(field.name.into(), schema);
                }
                if op.word == "answers" { properties.insert("messages".into(), json!({"type":"array", "minItems":1, "maxItems":32, "items":{"type":"object", "properties":{"role":{"type":"string", "enum":["user", "assistant", "system"]}, "content":{"type":"string"}}, "required":["role", "content"], "additionalProperties":false}, "description":"User/assistant/system messages; explicit alternative to q"})); properties.insert("stream".into(), json!({"const":false})); }
                ProviderCapability { id: id(op).parse().expect("static ID"), description: format!("Brave {} (fixed {} {})", op.word, if op.post { "POST" } else { "GET" }, op.path), effect: EffectKind::ReadOnly, risk: RiskLevel::Low,
                    input_schema: json!({"type":"object", "properties": properties, "additionalProperties": false}) }
            }).collect() }
    }
    fn run_command(argv: &[String], stdin: Option<&str>) -> Result<CommandRun, ProviderError> {
        run(argv, stdin)
    }
    fn invoke(capability: &CapabilityId, input: Value) -> Result<Value, ProviderError> {
        invoke_with(capability, input, dekopon_provider_http::send)
    }
}

fn tree() -> Command {
    let mut cmd = Command::new("bx")
        .about("Brave Search (buffered answers; broker-owned credentials)")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true);
    for op in OPS {
        let mut sub = Command::new(op.word);
        if op.word == "answers" {
            sub = sub.about("Buffered only; --no-stream accepted for bx compatibility; use '-' for stdin JSON messages");
        }
        if op.query {
            sub = sub.arg(Arg::new("q").required(true).value_name("QUERY"));
        } else if op.word == "places" {
            sub = sub.arg(Arg::new("q").value_name("QUERY"));
        }
        if matches!(op.word, "pois" | "descriptions") {
            sub = sub.arg(Arg::new("ids").num_args(1..).value_name("ID"));
        }
        for field in fields(*op) {
            let flag = field.name.replace('_', "-");
            // clap's command builder requires static identifiers; wire names are static.
            let mut arg = Arg::new(field.name)
                .long(Box::leak(flag.into_boxed_str()) as &'static str)
                .value_name("VALUE");
            match field.kind {
                Kind::List => {
                    arg = arg.action(ArgAction::Append);
                }
                Kind::Boolean => {
                    arg = arg
                        .num_args(0..=1)
                        .default_missing_value("true")
                        .value_parser(clap::builder::BoolishValueParser::new());
                }
                Kind::Number => {
                    arg = arg.value_parser(clap::value_parser!(u32));
                }
                Kind::Choice(choices) => {
                    arg = arg.value_parser(choices.to_vec());
                }
                Kind::Text => {}
            }
            if field.name == "long" {
                arg = arg.requires("lat").allow_hyphen_values(true);
            }
            if field.name == "lat" || field.name == "latitude" {
                arg = arg.allow_hyphen_values(true);
            }
            if field.name == "maximum_number_of_urls" {
                arg = arg.visible_alias("max-urls");
            }
            if field.name == "maximum_number_of_tokens" {
                arg = arg.visible_alias("max-tokens");
            }
            if field.name == "maximum_number_of_snippets" {
                arg = arg.visible_alias("max-snippets");
            }
            if field.name == "maximum_number_of_tokens_per_url" {
                arg = arg.visible_alias("max-tokens-per-url");
            }
            if field.name == "maximum_number_of_snippets_per_url" {
                arg = arg.visible_alias("max-snippets-per-url");
            }
            if field.name == "context_threshold_mode" {
                arg = arg.visible_alias("threshold");
            }
            if field.name == "longitude" {
                arg = arg.requires("latitude");
            }
            if field.name == "goggles" {
                arg = arg.conflicts_with_all(["include_site", "exclude_site"]);
            }
            if field.name == "include_site" {
                arg = arg.conflicts_with("exclude_site");
            }
            sub = sub.arg(arg);
        }
        if op.word == "answers" {
            sub = sub.arg(
                Arg::new("no-stream")
                    .long("no-stream")
                    .action(ArgAction::SetTrue),
            );
        }
        cmd = cmd.subcommand(sub);
    }
    cmd
}

fn run(argv: &[String], stdin: Option<&str>) -> Result<CommandRun, ProviderError> {
    let mut args = argv.to_vec();
    // Upstream bare-query shorthand. Only a positional first argument selects context;
    // `--` forces a query whose spelling equals a subcommand.
    if args
        .first()
        .is_some_and(|v| v == "--" || (!v.starts_with('-') && !OPS.iter().any(|o| o.word == v)))
    {
        args.insert(0, "context".into());
    }
    cli::run_command(tree(), &args, stdin, |matches, stdin| {
        let (name, sub) = matches.subcommand().expect("subcommand required");
        dispatch(name, sub, stdin)
    })
}

fn dispatch(
    name: &str,
    matches: &ArgMatches,
    stdin: Option<&str>,
) -> Result<CommandInvocation, ProviderError> {
    let op = OPS
        .iter()
        .find(|op| op.word == name)
        .ok_or_else(|| usage("unknown command"))?;
    let mut input = Map::new();
    if matches!(
        name,
        "context"
            | "web"
            | "news"
            | "images"
            | "videos"
            | "places"
            | "suggest"
            | "spellcheck"
            | "answers"
    ) && let Some(q) = matches.get_one::<String>("q")
    {
        if q == "-" && name == "answers" {
            let raw = stdin.ok_or_else(|| usage("answers - requires piped JSON"))?;
            if raw.len() > 64 * 1024 {
                return Err(usage("stdin JSON too large"));
            }
            let parsed: Value =
                serde_json::from_str(raw).map_err(|_| usage("invalid stdin JSON"))?;
            let object = parsed
                .as_object()
                .ok_or_else(|| usage("stdin JSON must be an object"))?;
            if object.contains_key("q") {
                return Err(usage("stdin answers must use messages"));
            }
            input.extend(object.clone());
            if fields(*op).any(|f| matches.contains_id(f.name)) {
                return Err(usage("stdin JSON and flags cannot be merged"));
            }
        } else {
            input.insert("q".into(), Value::String(q.clone()));
        }
    }
    if matches!(name, "pois" | "descriptions")
        && let Some(ids) = matches.get_many::<String>("ids")
    {
        input.insert("ids".into(), json!(ids.collect::<Vec<_>>()));
    }
    for field in fields(*op) {
        if let Some(value) = match field.kind {
            Kind::Boolean => matches.get_one::<bool>(field.name).map(|v| json!(v)),
            Kind::Number => matches.get_one::<u32>(field.name).map(|v| json!(v)),
            Kind::List => matches
                .get_many::<String>(field.name)
                .map(|v| {
                    Ok::<Value, ProviderError>(if field.name == "result_filter" {
                        json!(
                            v.flat_map(|value| value.split(',').map(str::trim).map(str::to_owned))
                                .collect::<Vec<_>>()
                        )
                    } else if field.name == "goggles" {
                        let mut rules = Vec::new();
                        for value in v {
                            if value == "@-" {
                                let piped = stdin
                                    .ok_or_else(|| usage("--goggles @- requires piped rules"))?;
                                if piped.is_empty() || piped.len() > MAX_TEXT {
                                    return Err(usage("piped Goggles must be 1-4096 bytes"));
                                }
                                rules.push(piped.to_owned());
                            } else {
                                rules.push(value.replace("\\n", "\n"));
                            }
                        }
                        json!(rules)
                    } else {
                        json!(v.collect::<Vec<_>>())
                    })
                })
                .transpose()?,
            _ => matches.get_one::<String>(field.name).map(|v| json!(v)),
        } {
            input.insert(field.name.into(), value);
        }
    }
    let value = Value::Object(input);
    validate(*op, &value)?;
    Ok(CommandInvocation {
        capability: id(*op).parse().expect("static ID"),
        input: value,
        secret_use: None,
    })
}

fn validate(op: Operation, input: &Value) -> Result<(), ProviderError> {
    let obj = input
        .as_object()
        .ok_or_else(|| usage("input must be an object"))?;
    for (key, value) in obj {
        if key == "q" && (op.query || op.word == "places") {
            let q = value.as_str().ok_or_else(|| usage("q must be text"))?;
            if q.trim().is_empty() || q.len() > MAX_QUERY {
                return Err(usage("q must be nonempty and at most 2048 bytes"));
            }
        } else if key == "ids" && matches!(op.word, "pois" | "descriptions") {
            let ids = value
                .as_array()
                .ok_or_else(|| usage("ids must be an array"))?;
            if ids.is_empty()
                || ids.len() > 20
                || ids
                    .iter()
                    .any(|v| v.as_str().is_none_or(|s| s.is_empty() || s.len() > 256))
            {
                return Err(usage("ids must contain 1-20 nonempty IDs"));
            }
        } else if key == "stream" && op.word == "answers" {
            if value != false {
                return Err(usage("streaming is not supported"));
            }
        } else if key == "messages" && op.word == "answers" {
            let messages = value
                .as_array()
                .ok_or_else(|| usage("messages must be an array"))?;
            if messages.is_empty()
                || messages.len() > 32
                || messages.iter().any(|v| {
                    !v.is_object()
                        || !matches!(
                            v.get("role").and_then(Value::as_str),
                            Some("user" | "assistant" | "system")
                        )
                        || v.get("content").and_then(Value::as_str).is_none()
                        || v.as_object().is_none_or(|message| message.len() != 2)
                })
            {
                return Err(usage("messages require only role and text content"));
            }
        } else if let Some(field) = fields(op).find(|f| f.name == key) {
            let valid = match field.kind {
                Kind::Text => value.as_str().is_some_and(|s| {
                    !s.is_empty() && s.len() <= MAX_TEXT && !s.chars().any(char::is_control)
                }),
                Kind::Choice(choices) => value.as_str().is_some_and(|s| choices.contains(&s)),
                Kind::Number => {
                    let (min, max) = number_bounds(op, field.name);
                    value.as_u64().is_some_and(|n| (min..=max).contains(&n))
                }
                Kind::Boolean => value.is_boolean(),
                Kind::List => value.as_array().is_some_and(|list| {
                    !list.is_empty()
                        && list.len() <= if field.name == "goggles" { 3 } else { 20 }
                        && list.iter().all(|v| {
                            v.as_str()
                                .is_some_and(|s| !s.is_empty() && s.len() <= MAX_TEXT)
                        })
                }),
            };
            if !valid {
                return Err(usage(format!("invalid {key}")));
            }
        } else {
            return Err(usage(format!("unsupported field {key}")));
        }
    }
    if op.query && !obj.contains_key("q") && !(op.word == "answers" && obj.contains_key("messages"))
    {
        return Err(usage("query required"));
    }
    if op.word == "answers" && obj.contains_key("q") && obj.contains_key("messages") {
        return Err(usage("q and messages are alternatives"));
    }
    if matches!(op.word, "pois" | "descriptions") && !obj.contains_key("ids") {
        return Err(usage("ids required"));
    }
    if op.word == "places"
        && !obj.contains_key("q")
        && !obj.contains_key("location")
        && !obj.contains_key("latitude")
    {
        return Err(usage("places requires query or location"));
    }
    for (longitude, latitude) in [("long", "lat"), ("longitude", "latitude")] {
        if obj.contains_key(longitude) && !obj.contains_key(latitude) {
            return Err(usage(format!("{longitude} requires {latitude}")));
        }
    }
    if obj.contains_key("goggles")
        && (obj.contains_key("include_site") || obj.contains_key("exclude_site"))
        || obj.contains_key("include_site") && obj.contains_key("exclude_site")
    {
        return Err(usage("goggles and site filters are mutually exclusive"));
    }
    if let Some(goggles) = obj.get("goggles")
        && goggles.as_array().is_some_and(|v| {
            v.iter().any(|s| {
                s.as_str().is_some_and(|s| {
                    s.starts_with('@') || s.starts_with("http://") || s.starts_with("https://")
                })
            })
        })
    {
        return Err(usage(
            "only inline Goggles are supported (no files or hosted URLs; CLI supports @- via piped stdin)",
        ));
    }
    if let Some(filter) = obj.get("result_filter")
        && filter.as_array().is_some_and(|items| {
            items.iter().any(|v| {
                !matches!(
                    v.as_str(),
                    Some(
                        "web"
                            | "discussions"
                            | "faq"
                            | "infobox"
                            | "news"
                            | "videos"
                            | "query"
                            | "summarizer"
                            | "locations"
                    )
                )
            })
        })
    {
        return Err(usage("invalid result filter"));
    }
    for key in ["include_site", "exclude_site"] {
        if let Some(values) = obj.get(key).and_then(Value::as_array)
            && values.iter().any(|v| {
                v.as_str().is_none_or(|domain| {
                    !domain
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
                })
            })
        {
            return Err(usage(format!("invalid {key} domain")));
        }
    }
    if op.word == "answers" && obj.get("q").and_then(Value::as_str) == Some("-") {
        return Err(usage("answers - requires stdin JSON through bx"));
    }
    Ok(())
}

fn percent(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            out.push(char::from(byte));
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}
fn query_param(url: &mut String, key: &str, value: &str) {
    url.push(if url.contains('?') { '&' } else { '?' });
    url.push_str(key);
    url.push('=');
    url.push_str(&percent(value));
}

fn invoke_with<F>(capability: &CapabilityId, input: Value, send: F) -> Result<Value, ProviderError>
where
    F: FnOnce(Request) -> Result<Response, HttpError>,
{
    let op = OPS
        .iter()
        .find(|op| id(**op) == capability.as_str())
        .ok_or_else(|| ProviderError::new("unknown-capability", "unsupported bx capability"))?;
    validate(*op, &input)?;
    let mut obj = input.as_object().expect("validated object").clone();
    let mut url = format!("{ORIGIN}{}", op.path);
    let mut headers = Vec::new();
    if op.location || op.word == "pois" {
        for (key, header) in [
            ("lat", "X-Loc-Lat"),
            ("long", "X-Loc-Long"),
            ("timezone", "X-Loc-Timezone"),
            ("city", "X-Loc-City"),
            ("state", "X-Loc-State"),
            ("state_name", "X-Loc-State-Name"),
            ("loc_country", "X-Loc-Country"),
            ("postal_code", "X-Loc-Postal-Code"),
        ] {
            if let Some(value) = obj.remove(key) {
                headers.push(
                    Header::text(header, value.as_str().expect("validated text"))
                        .map_err(|_| usage("invalid location header"))?,
                );
            }
        }
    }
    if op.word == "answers" {
        if let Some(q) = obj.remove("q") {
            obj.insert("messages".into(), json!([{"role":"user","content":q}]));
        }
        let mut approx = Map::new();
        for key in ["city", "country", "region", "timezone"] {
            if let Some(value) = obj.remove(&format!("user_{key}")) {
                approx.insert(key.into(), value);
            }
        }
        let mut wso = Map::new();
        if let Some(size) = obj.remove("search_context_size") {
            wso.insert("search_context_size".into(), size);
        }
        if !approx.is_empty() {
            wso.insert("user_location".into(), json!({"approximate":approx}));
        }
        if !wso.is_empty() {
            obj.insert("web_search_options".into(), Value::Object(wso));
        }
        obj.insert("stream".into(), Value::Bool(false));
    }
    if let Some(site) = obj.remove("include_site") {
        obj.insert(
            "goggles".into(),
            json!([format!(
                "$discard\n{}",
                site.as_array()
                    .expect("validated")
                    .iter()
                    .map(|v| format!("$boost,site={}", v.as_str().expect("validated")))
                    .collect::<Vec<_>>()
                    .join("\n")
            )]),
        );
    }
    if let Some(site) = obj.remove("exclude_site") {
        obj.insert(
            "goggles".into(),
            json!([site
                .as_array()
                .expect("validated")
                .iter()
                .map(|v| format!("$discard,site={}", v.as_str().expect("validated")))
                .collect::<Vec<_>>()
                .join("\n")]),
        );
    }
    if let Some(filter) = obj.remove("result_filter") {
        obj.insert(
            "result_filter".into(),
            json!(
                filter
                    .as_array()
                    .expect("validated")
                    .iter()
                    .map(|v| v.as_str().expect("validated"))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        );
    }
    if !op.post {
        for (key, value) in &obj {
            let key = if key == "loc_country" { "country" } else { key };
            if let Value::Array(values) = value {
                for value in values {
                    query_param(&mut url, key, value.as_str().expect("validated"));
                }
            } else {
                query_param(
                    &mut url,
                    key,
                    &if let Some(s) = value.as_str() {
                        s.to_owned()
                    } else {
                        value.to_string()
                    },
                );
            }
        }
    }
    let mut request = Request::new(if op.post { method::POST } else { method::GET }, &url)
        .map_err(|_| usage("invalid request"))?;
    if op.post {
        request = request
            .with_header(Header::text("content-type", "application/json").expect("static header"))
            .with_body(serde_json::to_vec(&obj).map_err(|_| usage("invalid input"))?);
    }
    for header in headers {
        request = request.with_header(header);
    }
    let response = send(request)
        .map_err(|_| ProviderError::new("http-failed", "broker HTTP request failed"))?;
    match response.status {
        200..=299 => serde_json::from_slice(&response.body)
            .map_err(|_| ProviderError::new("invalid-response", "Brave returned invalid JSON")),
        401 | 403 => Err(ProviderError::new(
            "unauthorized",
            "Brave refused authentication or access",
        )),
        429 => Err(ProviderError::new(
            "rate-limited",
            "Brave rate limit exceeded",
        )),
        _ => Err(ProviderError::new(
            "upstream-error",
            format!("Brave returned HTTP {}", response.status),
        )),
    }
}

dekopon_provider_sdk::export_provider_with_cli!(Brave, bindings);

#[cfg(test)]
mod tests {
    use super::*;
    fn argv(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| (*s).into()).collect()
    }
    fn proposed(args: &[&str], stdin: Option<&str>) -> (String, Value) {
        match run(&argv(args), stdin).expect("valid CLI") {
            CommandRun::Proposal(p) => (p.capability.to_string(), p.input),
            other => panic!("not proposed: {other:?}"),
        }
    }
    fn request(op: &str, input: Value) -> Request {
        let mut captured = None;
        let _ = invoke_with(&format!("bx.{op}").parse().unwrap(), input, |req| {
            captured = Some(req);
            Ok(Response {
                status: 200,
                headers: vec![],
                body: br#"{"results":[],"future":{"x":1}}"#.to_vec(),
            })
        })
        .unwrap();
        captured.unwrap()
    }
    #[test]
    fn all_commands_and_manifest_route_to_fixed_paths() {
        let manifest = Brave::manifest();
        assert_eq!(manifest.capabilities.len(), 11);
        let places = manifest
            .capabilities
            .iter()
            .find(|c| c.id.as_str() == "bx.places")
            .unwrap();
        assert_eq!(
            places.input_schema["properties"]["q"]["maxLength"],
            MAX_QUERY
        );
        let answers = manifest
            .capabilities
            .iter()
            .find(|c| c.id.as_str() == "bx.answers")
            .unwrap();
        assert_eq!(
            answers.input_schema["properties"]["messages"]["items"]["additionalProperties"],
            false
        );
        let (_, place_input) = proposed(&["places", "coffee", "--location", "NYC"], None);
        assert_eq!(place_input["q"], "coffee");
        assert_eq!(
            request("places", place_input).uri,
            "https://api.search.brave.com/res/v1/local/place_search?location=NYC&q=coffee"
        );
        for op in OPS {
            let mut args = vec![op.word];
            if op.query {
                args.push("test");
            }
            if op.word == "places" {
                args.push("--location");
                args.push("NYC");
            }
            if matches!(op.word, "pois" | "descriptions") {
                args.push("poi-1");
            }
            let (cap, input) = proposed(&args, None);
            assert_eq!(cap, id(*op));
            assert!(manifest.capabilities.iter().any(|c| c.id.as_str() == cap));
            let req = request(op.word, input);
            assert!(req.uri.starts_with(&format!("{ORIGIN}{}", op.path)));
            assert_eq!(req.method, if op.post { "POST" } else { "GET" });
            assert!(
                !req.headers
                    .iter()
                    .any(|h| h.name.eq_ignore_ascii_case("authorization")
                        || h.name.eq_ignore_ascii_case("x-subscription-token"))
            );
        }
    }
    #[test]
    fn rich_mappings_and_buffered_answers() {
        let (_, input) = proposed(
            &[
                "web",
                "rust",
                "--count",
                "7",
                "--result-filter",
                "web",
                "--result-filter",
                "news",
                "--include-site",
                "docs.rs",
                "--lat",
                "37",
                "--long",
                "-122",
                "--operators",
                "false",
            ],
            None,
        );
        let req = request("web", input);
        let body: Value = serde_json::from_slice(&req.body).unwrap();
        assert_eq!(body["result_filter"], "web,news");
        assert_eq!(body["goggles"], json!(["$discard\n$boost,site=docs.rs"]));
        assert_eq!(body["operators"], false);
        let (_, input) = proposed(&["web", "rust", "--result-filter", "web,news"], None);
        let body: Value = serde_json::from_slice(&request("web", input).body).unwrap();
        assert_eq!(body["result_filter"], "web,news");
        let body: Value =
            serde_json::from_slice(&request("web", json!({"q":"x", "count":20, "offset":9})).body)
                .unwrap();
        assert_eq!(body["count"], 20);
        assert_eq!(body["offset"], 9);
        assert!(req.headers.iter().any(|h| h.name == "X-Loc-Lat"));
        let (_, input) = proposed(
            &["answers", "-", "--no-stream"],
            Some(r#"{"messages":[{"role":"user","content":"test"}],"stream":false}"#),
        );
        let req = request("answers", input);
        let body: Value = serde_json::from_slice(&req.body).unwrap();
        assert_eq!(body["stream"], false);
        assert_eq!(body["messages"][0]["content"], "test");
    }
    #[test]
    fn invalid_inputs_and_help_never_egress() {
        for args in [
            &["--help"][..],
            &["web", "--help"],
            &["web"],
            &["config", "set-key"],
            &["web", "x", "--api-key", "secret"],
            &["answers", "x", "--enable-research"],
        ] {
            assert!(
                !matches!(run(&argv(args), None), Ok(CommandRun::Proposal(_))),
                "{args:?}"
            );
        }
        for (op, input) in [
            ("web", json!({"q":"x", "endpoint":"/admin"})),
            ("web", json!({"q":"x", "goggles":["@file"]})),
            (
                "answers",
                json!({"messages":[{"role":"tool","content":"x"}]}),
            ),
            ("places", json!({"longitude":"4"})),
            ("pois", json!({"ids":[]})),
        ] {
            let mut called = false;
            assert!(
                invoke_with(&format!("bx.{op}").parse().unwrap(), input, |_| {
                    called = true;
                    unreachable!()
                })
                .is_err()
            );
            assert!(!called);
        }
    }
    #[test]
    fn get_queries_are_encoded_and_repeated_ids_remain_ordered() {
        let (_, input) = proposed(
            &[
                "images",
                "café & cats",
                "--count",
                "4",
                "--spellcheck",
                "false",
            ],
            None,
        );
        let req = request("images", input);
        assert_eq!(
            req.uri,
            "https://api.search.brave.com/res/v1/images/search?count=4&q=caf%C3%A9%20%26%20cats&spellcheck=false"
        );
        assert!(req.body.is_empty());
        let (_, input) = proposed(
            &[
                "pois",
                "id/one",
                "id two",
                "--search-lang",
                "en",
                "--lat",
                "37",
                "--long",
                "-122",
            ],
            None,
        );
        let req = request("pois", input);
        assert_eq!(
            req.uri,
            "https://api.search.brave.com/res/v1/local/pois?ids=id%2Fone&ids=id%20two&search_lang=en"
        );
        assert_eq!(
            req.headers
                .iter()
                .find(|h| h.name == "X-Loc-Long")
                .unwrap()
                .value,
            b"-122"
        );
        let (_, input) = proposed(&["suggest", "a+b", "--rich", "false"], None);
        assert_eq!(
            request("suggest", input).uri,
            "https://api.search.brave.com/res/v1/suggest/search?q=a%2Bb&rich=false"
        );
    }
    #[test]
    fn context_and_news_preserve_native_post_options() {
        let (_, input) = proposed(
            &[
                "rust",
                "--max-tokens",
                "4096",
                "--threshold",
                "disabled",
                "--goggles",
                "$boost=3,site=docs.rs",
                "--goggles",
                "$downrank,site=example.com",
            ],
            None,
        );
        let req = request("context", input);
        assert_eq!(req.uri, "https://api.search.brave.com/res/v1/llm/context");
        let body: Value = serde_json::from_slice(&req.body).unwrap();
        assert_eq!(body["maximum_number_of_tokens"], 4096);
        assert_eq!(body["context_threshold_mode"], "disabled");
        assert_eq!(
            body["goggles"],
            json!(["$boost=3,site=docs.rs", "$downrank,site=example.com"])
        );
        let (_, input) = proposed(
            &["context", "x", "--goggles", "@-"],
            Some("$boost=3,site=docs.rs"),
        );
        let body: Value = serde_json::from_slice(&request("context", input).body).unwrap();
        assert_eq!(body["goggles"], json!(["$boost=3,site=docs.rs"]));
        let (_, input) = proposed(
            &[
                "news",
                "x",
                "--freshness",
                "pw",
                "--exclude-site",
                "example.com",
                "--spellcheck",
                "false",
            ],
            None,
        );
        let body: Value = serde_json::from_slice(&request("news", input).body).unwrap();
        assert_eq!(body["goggles"], json!(["$discard,site=example.com"]));
        assert_eq!(body["spellcheck"], false);
    }
    #[test]
    fn invalid_combinations_refuse_before_transport() {
        for args in [
            &[
                "web",
                "x",
                "--goggles",
                "$discard",
                "--include-site",
                "foo.com",
            ][..],
            &["context", "x", "--long", "0"],
            &["answers", "-"],
            &["context", "x", "--goggles", "@-"],
            &["web", "x", "--result-filter", "management"],
            &["web", "x", "--count", "0"],
            &["web", "x", "--count", "100000"],
            &["web", "x", "--offset", "100000"],
            &[
                "context",
                "x",
                "--goggles",
                "a",
                "--goggles",
                "b",
                "--goggles",
                "c",
                "--goggles",
                "d",
            ],
        ] {
            assert!(
                !matches!(run(&argv(args), None), Ok(CommandRun::Proposal(_))),
                "{args:?}"
            );
        }
        let injected = r#"{"messages":[{"role":"user","content":"x","extra":{"endpoint":"/admin"}}],"stream":false}"#;
        let error = run(&argv(&["answers", "-"]), Some(injected)).unwrap_err();
        assert_eq!(error.code(), "usage");
        assert_eq!(
            error.message(),
            "messages require only role and text content"
        );
        for (op, input) in [
            (
                "web",
                json!({"q":"x", "goggles":["https://evil.test/rules"]}),
            ),
            ("web", json!({"q":"x", "include_site":["evil.test/../"]})),
            ("web", json!({"q":"x", "lat":"1\r\nAuthorization: bad"})),
            ("answers", json!({"q":"x", "stream":true})),
            (
                "answers",
                json!({"q":"x", "messages":[{"role":"user","content":"y"}]}),
            ),
            ("images", json!({"q":"x", "safesearch":"moderate"})),
            ("web", json!({"q":"x", "count":0})),
            ("web", json!({"q":"x", "count":100000})),
            ("web", json!({"q":"x", "offset":100000})),
            ("context", json!({"q":"x", "goggles":["a", "b", "c", "d"]})),
            (
                "answers",
                json!({"messages":[{"role":"user","content":"x","extra":{"endpoint":"/admin"}}]}),
            ),
        ] {
            let mut called = false;
            assert!(
                invoke_with(&format!("bx.{op}").parse().unwrap(), input, |_| {
                    called = true;
                    unreachable!()
                })
                .is_err()
            );
            assert!(!called);
        }
    }
    #[test]
    fn documented_numeric_limits_match_manifest_and_invoke() {
        let manifest = Brave::manifest();
        for op in OPS {
            for field in fields(*op).filter(|f| f.kind == Kind::Number) {
                let (min, max) = number_bounds(*op, field.name);
                let capability = manifest
                    .capabilities
                    .iter()
                    .find(|c| c.id.as_str() == id(*op))
                    .unwrap();
                assert_eq!(
                    capability.input_schema["properties"][field.name]["minimum"], min,
                    "{}.{}",
                    op.word, field.name
                );
                assert_eq!(
                    capability.input_schema["properties"][field.name]["maximum"], max,
                    "{}.{}",
                    op.word, field.name
                );
                let mut base = match op.word {
                    "places" => json!({"q":"x"}),
                    "pois" | "descriptions" => json!({"ids":["synthetic-id"]}),
                    _ => json!({"q":"x"}),
                };
                for invalid in [min.checked_sub(1), max.checked_add(1)]
                    .into_iter()
                    .flatten()
                {
                    base[field.name] = json!(invalid);
                    let mut called = false;
                    assert!(
                        invoke_with(&id(*op).parse().unwrap(), base.clone(), |_| {
                            called = true;
                            unreachable!()
                        })
                        .is_err(),
                        "{}.{} accepted {}",
                        op.word,
                        field.name,
                        invalid
                    );
                    assert!(!called);
                }
            }
        }
    }
    #[test]
    fn response_and_error_handling() {
        let id = "bx.web".parse().unwrap();
        let input = json!({"q":"x"});
        let value = invoke_with(&id, input.clone(), |_| Ok(Response { status: 200, headers: vec![], body: br#"{"web":{"results":[{"url":"https://example.org","future":1}]},"future":true}"#.to_vec() })).unwrap();
        assert_eq!(value["web"]["results"][0]["url"], "https://example.org");
        for (status, code) in [
            (401, "unauthorized"),
            (403, "unauthorized"),
            (429, "rate-limited"),
            (500, "upstream-error"),
        ] {
            assert_eq!(
                invoke_with(&id, input.clone(), |_| Ok(Response {
                    status,
                    headers: vec![],
                    body: vec![]
                }))
                .unwrap_err()
                .code(),
                code
            );
        }
        assert_eq!(
            invoke_with(&id, input.clone(), |_| Ok(Response {
                status: 200,
                headers: vec![],
                body: b"not json".to_vec()
            }))
            .unwrap_err()
            .code(),
            "invalid-response"
        );
    }
}

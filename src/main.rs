use base64::{engine::general_purpose::STANDARD, Engine};
use minisign_verify::{PublicKey, Signature};
use semver::Version;
use serde_json::Value;
use std::{env, fs, process};

const KEY: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IDY2RUNBOTNERTU4NDk2RjIKUldUeWxvVGxQYW5zWnFOaG5IRFQxMHpYNXBXRFlBZ0IxK21ZMU10cklCZlhKdk45d2FDeFpza0UK";
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 { return Err("usage: controller prepare <channel.json> <policy.json> <payload.json> | finish <payload.json> <payload.sig> <planner.json>".into()); }
    if args[1] == "prepare" {
        let bytes = fs::read(&args[2])?;
        if bytes.len() > 1024 * 1024 { return Err("manifest too large".into()); }
        let raw: Value = serde_json::from_slice(&bytes)?;
        let signed = raw["signed_manifest"].as_str().ok_or("missing signed payload")?;
        let signature = raw["manifest_signature"].as_str().ok_or("missing signature")?;
        let public = PublicKey::decode(&String::from_utf8(STANDARD.decode(KEY.trim())?)?)?;
        let signature = Signature::decode(&String::from_utf8(STANDARD.decode(signature.trim())?)?)?;
        public.verify(signed.as_bytes(), &signature, false)?;
        let mut payload: Value = serde_json::from_str(signed)?;
        let mut outer = raw.clone();
        let outer = outer.as_object_mut().ok_or("invalid manifest")?;
        outer.remove("signed_manifest"); outer.remove("manifest_signature");
        if &payload != &Value::Object(outer.clone()) || payload["channel"] != "planner" { return Err("signed and outer manifest differ or channel invalid".into()); }
        let policy: Value = serde_json::from_slice(&fs::read(&args[3])?)?;
        let min = policy["minimum_supported_version"].as_str().ok_or("missing minimum_supported_version")?;
        let minimum = Version::parse(min)?;
        let target = Version::parse(payload["version"].as_str().ok_or("missing version")?)?;
        if !minimum.pre.is_empty() || !minimum.build.is_empty() || minimum > target { return Err("minimum must be a stable version no newer than the channel".into()); }
        payload["minimum_supported_version"] = Value::String(minimum.to_string());
        fs::write(&args[4], serde_json::to_vec(&payload)?)?;
    } else if args[1] == "finish" {
        let payload = fs::read_to_string(&args[2])?;
        let mut outer: Value = serde_json::from_str(&payload)?;
        let signature = fs::read_to_string(&args[3])?;
        let public = PublicKey::decode(&String::from_utf8(STANDARD.decode(KEY.trim())?)?)?;
        let parsed = Signature::decode(&String::from_utf8(STANDARD.decode(signature.trim())?)?)?;
        public.verify(payload.as_bytes(), &parsed, false)?;
        outer["signed_manifest"] = Value::String(payload);
        outer["manifest_signature"] = Value::String(signature.trim().into());
        fs::write(&args[4], serde_json::to_vec_pretty(&outer)?)?;
    } else { return Err("unknown operation".into()); }
    Ok(())
}
fn main() { if let Err(err) = run() { eprintln!("policy update failed: {err}"); process::exit(1); } }

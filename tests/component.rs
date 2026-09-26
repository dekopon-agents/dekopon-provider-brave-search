// SPDX-License-Identifier: MPL-2.0
//! Real component tests: no HTTP grant or vendor credential is installed here.
use dekopon_provider_sdk_testkit::{
    BrokerHostError, CommandRunOutcome, FakeBroker, FakeBrokerError,
};
use serde_json::json;
use std::path::PathBuf;

#[tokio::test(flavor = "multi_thread")]
async fn component_exports_narrow_capabilities_and_refuses_ungranted_http()
-> Result<(), Box<dyn std::error::Error>> {
    let component = PathBuf::from(
        std::env::var_os("DEKOPON_PROVIDER_COMPONENT")
            .expect("DEKOPON_PROVIDER_COMPONENT must point to ./build.sh output"),
    );
    let broker = FakeBroker::builder()
        .component(component)
        .provider("bx")
        .build()
        .await?;
    let manifests: Vec<_> = broker.registry().manifests().collect();
    assert_eq!(manifests.len(), 1);
    assert_eq!(manifests[0].command_words, ["bx"]);
    assert_eq!(manifests[0].capabilities.len(), 11);
    assert!(
        manifests[0]
            .capabilities
            .iter()
            .all(|c| c.id.as_str().starts_with("bx."))
    );
    for args in [
        ["--help"].as_slice(),
        ["web", "--help"].as_slice(),
        ["web", "--api-key", "secret"].as_slice(),
    ] {
        let result = broker
            .run_command(
                "bx",
                &args.iter().map(|s| (*s).into()).collect::<Vec<_>>(),
                None,
            )
            .await?;
        assert!(matches!(result, CommandRunOutcome::Rendered { .. }));
    }
    let proposed = broker
        .run_command("bx", &["web".into(), "rust".into()], None)
        .await?;
    let CommandRunOutcome::Proposed {
        capability, input, ..
    } = proposed
    else {
        panic!("web should propose");
    };
    assert_eq!(capability.as_str(), "bx.web");
    assert_eq!(input, json!({"q":"rust"}));
    assert_eq!(
        broker
            .invoke("bx.web", json!({"q":"x", "endpoint":"/admin"}))
            .await
            .unwrap_err()
            .provider_failure()
            .map(|(code, _)| code),
        Some("usage")
    );
    let denied = broker.invoke("bx.web", input).await.unwrap_err();
    assert!(matches!(denied, FakeBrokerError::Invocation(failure)
        if matches!(failure.error.as_ref(), BrokerHostError::HostCallRejected { reason: "denied", .. })));
    Ok(())
}

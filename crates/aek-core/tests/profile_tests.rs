use aek_core::profile::{ProfileDescriptor, ProfileKind};

#[test]
fn test_profile_kinds() {
    assert_eq!(ProfileKind::all().len(), 3);
    assert_eq!(ProfileKind::Light.as_str(), "light");
    assert_eq!(ProfileKind::Engineering.as_str(), "engineering");
    assert_eq!(ProfileKind::Full.as_str(), "full");

    assert_eq!("light".parse::<ProfileKind>().unwrap(), ProfileKind::Light);
    assert_eq!(
        "engineering".parse::<ProfileKind>().unwrap(),
        ProfileKind::Engineering
    );
    assert_eq!("full".parse::<ProfileKind>().unwrap(), ProfileKind::Full);
    assert!("invalid_profile".parse::<ProfileKind>().is_err());
}

#[test]
fn test_profile_descriptor_metadata() {
    let desc = ProfileDescriptor::from(ProfileKind::Engineering);
    assert_eq!(desc.kind, ProfileKind::Engineering);
    assert!(!desc.description.is_empty());
    assert!(!desc.target_audience.is_empty());
    assert!(desc.features.len() >= 3);
}

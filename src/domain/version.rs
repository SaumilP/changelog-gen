use semver::Version;

pub fn bump_patch(version: &str) -> String {
    let mut v = Version::parse(version).unwrap();
    v.patch += 1;
    v.to_string()
}

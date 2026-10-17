#!/usr/bin/env bash
# Sign only on an ephemeral macOS runner. Credentials are supplied as secrets.
set -euo pipefail
: "${APPLE_CERT_BASE64:?Missing Developer ID certificate (PKCS12, base64)}"
: "${APPLE_CERT_PASSWORD:?Missing certificate password}"
: "${APPLE_SIGNING_IDENTITY:?Missing Developer ID Application identity}"
: "${APPLE_ID:?Missing notarization Apple ID}"
: "${APPLE_TEAM_ID:?Missing notarization team ID}"
: "${APPLE_APP_PASSWORD:?Missing notarization app-specific password}"
stage="$(cd -- "${1:?Usage: sign_macos.sh staging-directory}" && pwd)"
scratch="$(mktemp -d)"
keychain="$scratch/release.keychain-db"
password="$(openssl rand -hex 24)"
cleanup() {
    security delete-keychain "$keychain" >/dev/null 2>&1 || true
    rm -rf -- "$scratch"
}
trap cleanup EXIT
printf '%s' "$APPLE_CERT_BASE64" | base64 --decode > "$scratch/certificate.p12"
security create-keychain -p "$password" "$keychain"
security set-keychain-settings -lut 21600 "$keychain"
security unlock-keychain -p "$password" "$keychain"
security import "$scratch/certificate.p12" -k "$keychain" -P "$APPLE_CERT_PASSWORD" -T /usr/bin/codesign >/dev/null
security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k "$password" "$keychain" >/dev/null
for profile in debug release; do
    framework="$stage/addon/lib/universal-apple-darwin/$profile/spire_tween.framework"
    codesign --force --timestamp --options runtime --keychain "$keychain" --sign "$APPLE_SIGNING_IDENTITY" "$framework"
    codesign --verify --strict "$framework"
    ditto -c -k --keepParent "$framework" "$scratch/$profile.zip"
    xcrun notarytool submit "$scratch/$profile.zip" --apple-id "$APPLE_ID" --team-id "$APPLE_TEAM_ID" \
        --password "$APPLE_APP_PASSWORD" --wait --timeout 20m --output-format json > "$stage/notarization-$profile.json"
done
# Frameworks cannot be stapled like .app/.dmg/.pkg bundles. Acceptance records
# are preserved; Gatekeeper can retrieve the ticket for the signed code online.
cargo run --locked -p spire_release -- seal-signed-macos --stage "$stage"

#!/usr/bin/env bash
# Builds NX-Horizon's APK without Gradle: libhorizon.so with cargo-ndk, then aapt2, zipalign
# and apksigner from the Android SDK. Needs ANDROID_HOME and ANDROID_NDK_HOME.
#
#   packaging/android/build_apk.sh [--abi arm64-v8a|x86_64]... [--profile release|fast] [--out DIR]
#
# Signs with a debug key (created once in ~/.android/debug.keystore) unless KEYSTORE,
# KEYSTORE_PASS and KEY_ALIAS name a release key. An update installs over the app only when
# it is signed with the same key and its VERSION_CODE (default 1) is not lower.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../.." && pwd)"
abis=()
profile=release
out="$root/target/android"
while [ $# -gt 0 ]; do
    case "$1" in
        --abi) abis+=("$2"); shift 2 ;;
        --profile) profile="$2"; shift 2 ;;
        --out) out="$2"; shift 2 ;;
        *) echo "unknown argument: $1" >&2; exit 2 ;;
    esac
done
[ ${#abis[@]} -eq 0 ] && abis=(arm64-v8a)
: "${ANDROID_HOME:?set ANDROID_HOME to the Android SDK}"
: "${ANDROID_NDK_HOME:?set ANDROID_NDK_HOME to the Android NDK}"
platform="$ANDROID_HOME/platforms/android-35/android.jar"
tools="$(ls -d "$ANDROID_HOME"/build-tools/* | sort -V | tail -1)"
[ -f "$platform" ] || { echo "missing $platform (sdkmanager 'platforms;android-35')" >&2; exit 1; }

work="$out/work"
rm -rf "$work"
mkdir -p "$work/res/mipmap-xxxhdpi" "$work/res/mipmap-xxhdpi" "$work/res/mipmap-xhdpi" "$work/apk/lib"

# 1. The game library, one per ABI.
targets=()
for abi in "${abis[@]}"; do targets+=(-t "$abi"); done
(cd "$root" && cargo ndk "${targets[@]}" -P 28 -o "$work/apk/lib" build --profile "$profile" -p horizon-android)

# 2. Resources: the launcher icon from the desktop icon set.
icons="$root/packaging/icons/hicolor"
cp "$icons/256x256/apps/horizon.png" "$work/res/mipmap-xxxhdpi/icon.png"
cp "$icons/128x128/apps/horizon.png" "$work/res/mipmap-xxhdpi/icon.png"
cp "$icons/128x128/apps/horizon.png" "$work/res/mipmap-xhdpi/icon.png"
"$tools/aapt2" compile --dir "$work/res" -o "$work/res.zip"
version_args=(--version-code "${VERSION_CODE:-1}")
[ -n "${VERSION_NAME:-}" ] && version_args+=(--version-name "$VERSION_NAME")
"$tools/aapt2" link -o "$work/unaligned.apk" -I "$platform" \
    --manifest "$here/AndroidManifest.xml" "$work/res.zip" --auto-add-overlay \
    --replace-version "${version_args[@]}"

# 3. The libraries, stored uncompressed so Android maps them straight from the APK.
(cd "$work/apk" && zip -q -0 -r "$work/unaligned.apk" lib)
"$tools/zipalign" -f -P 16 4 "$work/unaligned.apk" "$work/aligned.apk"

# 4. Sign.
if [ -n "${KEYSTORE:-}" ]; then
    store="$KEYSTORE"; pass="${KEYSTORE_PASS:?}"; alias="${KEY_ALIAS:?}"
else
    store="$HOME/.android/debug.keystore"; pass=android; alias=androiddebugkey
    if [ ! -f "$store" ]; then
        mkdir -p "$(dirname "$store")"
        keytool -genkeypair -keystore "$store" -storepass android -keypass android \
            -alias androiddebugkey -keyalg RSA -keysize 2048 -validity 10000 \
            -dname "CN=Android Debug,O=Android,C=US" >/dev/null
    fi
fi
apk="$out/nx-horizon-$(IFS=-; echo "${abis[*]}").apk"
"$tools/apksigner" sign --ks "$store" --ks-pass "pass:$pass" --ks-key-alias "$alias" \
    --out "$apk" "$work/aligned.apk"
"$tools/apksigner" verify "$apk"
echo "$apk"

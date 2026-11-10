#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
ROOT=$PWD
SDK=${ANDROID_HOME:-$HOME/Android/Sdk}
export ANDROID_HOME=$SDK
export ANDROID_NDK_ROOT=${ANDROID_NDK_ROOT:-$(ls -d "$SDK"/ndk/* | sort -V | tail -1)}
export ANDROID_NDK_HOME=$ANDROID_NDK_ROOT
BT=$(ls -d "$SDK"/build-tools/* | sort -V | tail -1)
JAR=$(ls -d "$SDK"/platforms/android-3[4-9]* "$SDK"/platforms/android-34 2>/dev/null | sort -V | tail -1)/android.jar
OUT=$ROOT/target/android
if [ -n "${RELEASE:-}" ]; then PROFILE=release; FLAG=--release; DEBUGGABLE=false; else PROFILE=debug; FLAG=; DEBUGGABLE=true; fi
ABI=${ABI:-arm64-v8a}
case "$ABI" in arm64-v8a|x86_64) ;; *) echo "ABI must be arm64-v8a or x86_64" >&2; exit 1 ;; esac

JNI=$ROOT/target/android-jni
rm -rf "$JNI"
cargo ndk --platform 26 -t "$ABI" -o "$JNI" build --lib $FLAG
LIB=$JNI/$ABI/libcalcbench.so
[ -f "$LIB" ] || { echo "cargo ndk did not produce $LIB" >&2; exit 1; }

rm -rf "$OUT"; mkdir -p "$OUT"
VERSION=$(cargo pkgid | sed 's/.*[#@]//')
IFS=. read -r MAJOR MINOR PATCH <<<"${VERSION%%[-+]*}"
CODE=$((MAJOR * 10000 + MINOR * 100 + PATCH))
sed "s/@DEBUGGABLE@/$DEBUGGABLE/" packaging/android/AndroidManifest.xml > "$OUT/AndroidManifest.xml"
"$BT/aapt2" compile --dir android/res -o "$OUT/res.zip"
"$BT/aapt2" link -o "$OUT/base.apk" -I "$JAR" --manifest "$OUT/AndroidManifest.xml" \
    --min-sdk-version 26 --target-sdk-version 34 \
    --version-code "$CODE" --version-name "$VERSION" "$OUT/res.zip"
python3 - "$OUT" "$LIB" "$ABI" <<'PY'
import sys, zipfile
out, lib, abi = sys.argv[1:4]
with zipfile.ZipFile(f"{out}/base.apk", "a") as z:
    z.write(lib, f"lib/{abi}/libcalcbench.so", zipfile.ZIP_STORED)
PY
"$BT/zipalign" -f -p 4 "$OUT/base.apk" "$OUT/aligned.apk"
if [ -n "${KEYSTORE:-}" ]; then
    "$BT/apksigner" sign --ks "$KEYSTORE" --ks-pass "pass:${KEYSTORE_PASS:?}" \
        --key-pass "pass:${KEY_PASS:-$KEYSTORE_PASS}" \
        ${KEY_ALIAS:+--ks-key-alias "$KEY_ALIAS"} --out "$OUT/calcbench.apk" "$OUT/aligned.apk"
else
    "$BT/apksigner" sign --ks "$HOME/.android/debug.keystore" --ks-pass pass:android \
        --key-pass pass:android --out "$OUT/calcbench.apk" "$OUT/aligned.apk"
fi
rm -f "$OUT/calcbench.apk.idsig"
LISTING=$(unzip -l "$OUT/calcbench.apk")
grep -q "lib/$ABI/libcalcbench.so" <<<"$LISTING" || { echo "APK has no native library" >&2; exit 1; }
BADGING=$("$BT/aapt2" dump badging "$OUT/calcbench.apk")
if [ -n "${RELEASE:-}" ]; then
    ! grep -q 'application-debuggable' <<<"$BADGING" || { echo "release APK is debuggable" >&2; exit 1; }
fi
echo "built $OUT/calcbench.apk"

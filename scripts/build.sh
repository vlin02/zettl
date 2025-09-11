#!/bin/bash

APP_CERTIFICATE="3rd Party Mac Developer Application: Victor Lin (9W5UR2R6B9)"
APP_NAME="Zettl"

task darwin:package

cp ./zettl.provisionprofile "./bin/$APP_NAME.app/Contents"

codesign --timestamp --options=runtime -s "$APP_CERTIFICATE" -v --entitlements ./build/darwin/entitlements.plist "./bin/$APP_NAME.app"

PKG_CERTIFICATE="3rd Party Mac Developer Installer: Victor Lin (9W5UR2R6B9)"
productbuild --sign "$PKG_CERTIFICATE" --component "./bin/$APP_NAME.app" /Applications "./$APP_NAME.pkg"
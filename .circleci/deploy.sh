#!/usr/bin/env bash

set -e

VERSION_BASE=$(janus version -format='v%M.%m.x')
CLI_ARCHIVE_NAME="emerald-vault-osx-$APP_VERSION"
mv target/release/emerald-vault ./emerald-vault
zip -j "$CLI_ARCHIVE_NAME.zip" emerald-vault
tar -zcf "$CLI_ARCHIVE_NAME.tar.gz" emerald-vault
echo "Deploy to http://builds.etcdevteam.com/emerald-vault/$VERSION_BASE/"

mkdir deploy
mv *.zip *.tar.gz deploy/
ls -l deploy/

janus deploy -to="builds.etcdevteam.com/emerald-vault/$VERSION_BASE/" -files="deploy/*" -key=".circleci/gcloud-circleci.json.enc" -gpg
echo "Deployed"

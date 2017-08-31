#!/usr/bin/env bash

CLI_ARCHIVE_NAME="geth-classic-$TRAVIS_OS_NAME-$(janus version -format='TAG_OR_NIGHTLY')"
zip "$CLI_ARCHIVE_NAME.zip" emerald
tar -zcf "$CLI_ARCHIVE_NAME.tar.gz" emerald

mkdir deploy
mv *.zip *.tar.gz deploy/
ls -l deploy/

janus deploy -to="builds.etcdevteam.com/emerald-cli/$(janus version -format='v%M.%m.x')/" -files="./deploy/*" -key="./gcloud-travis.json.enc"
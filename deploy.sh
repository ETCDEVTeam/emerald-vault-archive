#!/usr/bin/env bash

VERSION_BASE=$(janus version -format='v%M.%m.x')
CLI_ARCHIVE_NAME="emerald-cli-$TRAVIS_OS_NAME-$VERSION_BASE"
zip "$CLI_ARCHIVE_NAME.zip" emerald
tar -zcf "$CLI_ARCHIVE_NAME.tar.gz" emerald

mkdir deploy
mv *.zip *.tar.gz deploy/
ls -l deploy/

janus deploy -to="builds.etcdevteam.com/emerald-cli/$VERSION_BASE/" -files="./deploy/*" -key="./gcloud-travis.json.enc"
#!/bin/sh
set -e

cd /data
ln -sfn /app/dist dist
[ -f settings.toml ] || cp /app/settings.toml settings.toml

exec /app/navalyze "$@"

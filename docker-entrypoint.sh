#!/bin/sh
set -e

cd /data
exec /app/navalyze -d /app/dist

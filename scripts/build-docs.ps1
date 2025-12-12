# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

pnpm install --no-frozen-lockfile
pnpm run -F docs build

New-Item './docs/dist/CNAME' -Force -ItemType 'file' -Value 'nil.dev.br'
New-Item './docs/dist/.nojekyll' -Force -ItemType 'file'

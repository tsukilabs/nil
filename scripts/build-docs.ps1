# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

pnpm install
pnpm run -F docs build

pnpm run -F nsr update
pnpm run -F nsr build

New-Item './docs/dist/CNAME' -Force -ItemType 'file' -Value 'nil.dev.br'
New-Item './docs/dist/.nojekyll' -Force -ItemType 'file'

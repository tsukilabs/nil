// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { SavedataFile } from '@/core/savedata';

export async function handleProcessArgs() {
  if (__DESKTOP__ && !__DEBUG_ASSERTIONS__) {
    const args = await commands.args();
    if (args[1]) {
      if (await commands.isSavedata(args[1])) {
        const savedata = await SavedataFile.read(args[1]);
        await savedata.load();
      }
      else if (await commands.isScript(args[1])) {
        await commands.importScript(args[1]);
      }
    }
  }
}

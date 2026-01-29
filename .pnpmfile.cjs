const { env } = require('node:process');

function readPackage(pkg) {
  if (env.NIL_MAP_WORKSPACE_DEPS === 'true' && pkg.dependencies) {
    const dependencies = {};
    for (const [key, value] of Object.entries(pkg.dependencies)) {
      dependencies[key] = key.startsWith('@tsukilabs/nil') ? 'latest' : value;
    }

    pkg.dependencies = dependencies;
  }

  return pkg;
}

module.exports = {
  hooks: {
    readPackage,
  },
};

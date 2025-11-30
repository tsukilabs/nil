const { env } = require('node:process');

function readPackage(pkg) {
  if (env.MAP_WORKSPACE_DEPS === 'true' && pkg.dependencies) {
    const dependencies = {};
    const version = `^${require('./package.json').version}`;
    for (const [key, value] of Object.entries(pkg.dependencies)) {
      dependencies[key] = key.startsWith('@tsukilabs/nil') ? version : value;
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

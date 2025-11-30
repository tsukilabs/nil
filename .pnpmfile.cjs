const { env } = require('node:process');

function readPackage(pkg) {
  if (env.GITHUB_CI === 'true' && pkg.dependencies) {
    const dependencies = {};
    for (const [key, value] of Object.entries(pkg.dependencies)) {
      dependencies[key] = key.startsWith('@tsukilabs') ? 'latest' : value;
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

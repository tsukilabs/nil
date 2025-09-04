const { env } = require('node:process');

function readPackage(pkg) {
  if (env.GITHUB_CI === 'true' && pkg.dependencies) {
    const dependencies = {};
    for (const [key, value] of Object.entries(pkg.dependencies)) {
      if (key.startsWith('@tsukilabs/')) {
        value = 'latest';
      }

      dependencies[key] = value;
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

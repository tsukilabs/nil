/** @type {import('typedoc').TypeDocOptions} */
export default {
  entryPoints: ["src/index.ts"],
  out: "../../docs/dist/js-docs/i18n",
  readme: "../../README.md",
  favicon: "../../docs/dist/favicon.png",
  includeVersion: true,
  excludeInternal: true,
  excludePrivate: true,
  excludeProtected: true,
  githubPages: false,
  hideGenerator: true,
  markdownLinkExternal: true,
  sourceLinkExternal: true,
  treatWarningsAsErrors: true,
};

/** @type {import('typedoc').TypeDocOptions} */
export default {
  entryPoints: ["src/index.ts"],
  out: "../../docs/.vitepress/dist/js-docs/ffi",
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

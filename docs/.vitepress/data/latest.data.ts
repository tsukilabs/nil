export default {
  async load() {
    const response = await fetch(
      'https://github.com/tsukilabs/nil/releases/latest/download/latest.json',
    );

    return response.json();
  },
};

export interface Latest {
  readonly version: string;
  readonly pub_date: string;
  readonly platforms: {
    readonly 'linux-x86_64-deb': { readonly url: string; };
    readonly 'linux-x86_64-rpm': { readonly url: string; };
    readonly 'windows-x86_64-nsis': { readonly url: string; };
  };
}

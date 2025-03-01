type JoinOptions = {
  readonly player: PlayerOptions;
  readonly serverAddr: import('@/lib/net/addr-v4').SocketAddrV4;
};

type HostOptions = {
  readonly player: PlayerOptions;
  readonly world: WorldOptions;
};

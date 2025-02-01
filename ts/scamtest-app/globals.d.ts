export {};

declare global {
  interface Window {
    CONFIG: Record<
      string,
      {
        scamtest: {
          package: string;
          scamtest: string;
        };
        slotUrl: string;
      }
    >;
  }
}

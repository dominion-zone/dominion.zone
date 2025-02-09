export type ScamtestConfig = {
  package: string;
  upgradeCap: string;
  adminCap: string;
  scamtest: string;
  tstCap: string;
};

export type Config = {
  scamtest: ScamtestConfig;
};

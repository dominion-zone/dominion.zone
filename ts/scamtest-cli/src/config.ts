export type ScamtestConfig = {
  package: string;
  upgradeCap: string;
  adminCap: string;
  scamtest: string;
};

export type Config = {
  scamtest: ScamtestConfig;
};

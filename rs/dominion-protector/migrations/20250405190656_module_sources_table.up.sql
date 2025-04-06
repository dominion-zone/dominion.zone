CREATE TABLE IF NOT EXISTS module_sources (
    package_id      CHAR(66) NOT NULL,
    network         VARCHAR(10) NOT NULL,
    module_name     TEXT NOT NULL,
    source          TEXT NOT NULL,
    kind            VARCHAR(20) NOT NULL,
    PRIMARY KEY(package_id, network, module_name),
    FOREIGN KEY(package_id, network)
        REFERENCES objects(object_id, network)
        ON DELETE CASCADE,
    FOREIGN KEY(package_id, network, module_name)
        REFERENCES package_modules(package_id, network, module_name)
        ON DELETE CASCADE
);
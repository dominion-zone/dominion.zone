CREATE TABLE package_linkage (
    package_id      CHAR(66) NOT NULL,
    network         VARCHAR(10) NOT NULL,
    dependency_id   CHAR(66) NOT NULL,
    upgraded_id     CHAR(66) NOT NULL,
    upgraded_version BIGINT NOT NULL,
    
    PRIMARY KEY(package_id, network, dependency_id),
    FOREIGN KEY(package_id, network)
        REFERENCES objects(object_id, network)
        ON DELETE CASCADE
);

CREATE TYPE OwnerType AS ENUM ('AddressOwner', 'ObjectOwner', 'Shared', 'Immutable', 'ConsensusV2');

CREATE TABLE objects (
    object_id              CHAR(66) NOT NULL,
    network                VARCHAR(10) NOT NULL,
    version                BIGINT NOT NULL,
    digest                 VARCHAR(64) NOT NULL,
    object_type            TEXT NOT NULL,
    owner_type             OwnerType NOT NULL,
    owner                  VARCHAR(66),
    initial_shared_version BIGINT,
    read_at                TIMESTAMPTZ NOT NULL DEFAULT Now(),

    PRIMARY KEY(object_id, network)
);

DROP DATABASE iotanlytcs;

CREATE DATABASE iotanlytcs;

CREATE USER iotanlytcs WITH ENCRYPTED PASSWORD 'iotanlytcs';

GRANT ALL PRIVILEGES ON DATABASE iotanlytcs TO iotanlytcs;

# Para desbloquear una base de datos en uso.
ALTER DATABASE iotanlytcs CONNECTION
LIMIT
    0;

SELECT
    pg_terminate_backend(pid)
FROM
    pg_stat_activity
WHERE
    datname = 'iotanlytcs';
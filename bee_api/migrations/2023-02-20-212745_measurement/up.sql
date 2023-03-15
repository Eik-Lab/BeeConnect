CREATE TABLE
  hive_infos (
    hive_id INTEGER NOT NULL,
    info TEXT NOT NULL,
    longitude FLOAT4 NOT NULL,
    altitude FLOAT4 NOT NULL,
    PRIMARY KEY (hive_id)
  );

CREATE TABLE measurements (
  id UUID NOT NULL,
  hive_id INTEGER NOT NULL,
  layer INTEGER NOT NULL,
  time timestamptz NOT NULL,
  weight FLOAT4,
  temp FLOAT4[] NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (hive_id) REFERENCES hive_infos(hive_id) 
); 

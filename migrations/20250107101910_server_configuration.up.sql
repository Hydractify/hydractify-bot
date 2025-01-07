CREATE TABLE IF NOT EXISTS server_configuration (
  guild_id            bigint  PRIMARY KEY,

  -- Where the starboard of that server lives in
  starboard_channel   bigint
);

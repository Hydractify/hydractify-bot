CREATE TABLE IF NOT EXISTS starboard (
  -- The message that got starred.
  message_id    bigint  PRIMARY KEY,

  -- The starboard message.
  starboard_id  bigint,

  -- How many stars it got.
  stars         integer NOT NULL
);

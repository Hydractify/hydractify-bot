CREATE TABLE IF NOT EXISTS starboard (
  -- The message that got starred.
  message_id    bigint  NOT NULL,

  -- The starboard message.
  starboard_id  bigint,

  -- How many stars it got.
  stars         integer NOT NULL
);

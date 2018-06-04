CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  email TEXT NOT NULL,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (email, username)
);

 INSERT INTO users (id, email, username, password, created_at) VALUES
  (1, 'admin@163.com', 'admin', '$2y$12$yXTjrGePVLBPUH6YVs2f5OsUEGSotZxdL5Uu/70r63I5GtynVVjkK', '2017-09-08 13:00:26.353041'),
  (2, 'aaaa@163.com', 'aaaa', '$2y$12$3lOwd/qun2g.KBQpYz7DQu4HgreLODO4aJgYwFAQNj2AqgS14DAMK', '2017-09-08 13:00:28.353041'),
  (3, 'zzzz@163.com', 'zzzz', '$2y$12$6ofSZ3hpsGtDt6bM0WU0geDgZLLETFUVB6FpMXI61SbAvuQD5RiWK', '2017-09-08 13:00:38.353041');
 SELECT setval('users_id_seq', 3, true);


 CREATE TABLE card (
   id SERIAL NOT NULL PRIMARY KEY,
   user_id INTEGER NOT NULL,
   front TEXT NOT NULL,
   back TEXT NOT NULL,
   body TEXT NOT NULL,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
 );

  INSERT INTO card (id, user_id, front, back, body, created_at) VALUES
  (1, 1, 'Topic',  'Test Front', 'Test Back', '2017-07-24 23:41:45.672805609');
  SELECT setval('card_id_seq', 1, true);-- Your SQL goes here

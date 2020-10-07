CREATE TABLE position (
	id SERIAL NOT NULL PRIMARY KEY,
  	longitude REAL NOT NULL,
  	latitude REAL NOT NULL
);

INSERT INTO position (
 longitude, latitude) VALUES 
 (1.2167, 44.3333),
 (1.48494, 39.0233);


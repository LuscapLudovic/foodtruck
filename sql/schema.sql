CREATE TABLE position (
	id SERIAL NOT NULL PRIMARY KEY,
  	longitude REAL NOT NULL,
  	latitude REAL NOT NULL,
  	nom VARCHAR(200) NOT NULL
);

INSERT INTO position (
 longitude, latitude, nom) VALUES 
(1.21669996,	44.3333015, 'las food truck'),
(1.48493505,	39.0232582, 'le camtar'),
(5.28332996,	45.1500015, 'chauffe marcel'),
(1.78332996,	49, 'au tacos'),
(1.68677902,	46.8114357, 'ça roulemapoul'),
(-0.423220009, 46.827179, 'genshin impact'),
(2.51666999,	49, 'ala grosse merguez'),
(2.28329992,	48.9000015, 'au food truck, vegan, gluten free, sans organismes génétiquement modifiés, local du roi des légumes des fruits et de la forêt'),
(2.73888898,	47.4300003, '231 burger'),
(-1.546628,	47.2124252, 'Suppli factory'),
(2.33216405,	48.8841667, 'burger quiz')


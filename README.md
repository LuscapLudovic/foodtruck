# foodtruck
je prend 30%

## Instruction d'installation

1. Créer un utilisateur
    ```shell
    createuser -P food_user
    ```
La première partie n'est pas indispensable si vous avez déjà un utilisateur de créer.    

2. Créer la base de données.

    ```shell
    createdb -O food_user foodtruck
    ```

Entrez votre mot de passe et suivez les instructions.
ici "food_user" est votre identifiant de pgsql, vous pouvez le remplacer par le votre.

3. Initialiser la base de données.

    ```shell
    psql -f sql/schema.sql foodtruck
    ```
    
Cela va créer la table et la remplir avec des données de test.

4. Créer le fichier '.env':
  ```ini
  SERVER_ADDR=127.0.0.1:8080
  PG.USER=password
  PG.PASSWORD=walahJeDonnePasMonMdp
  PG.HOST=127.0.0.1
  PG.PORT=5432
  PG.DBNAME=foodtruck
  PG.POOL.MAX_SIZE=16
  ```
  
5. Lancer le serveur:

  ```shell
  cargo run
  ```
  
6. Tester à l'adresse http://127.0.0.1:8080/position  






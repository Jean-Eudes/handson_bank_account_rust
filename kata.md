# Kata

## Présentation du projet

L'objectif de ce kata est de construire une banque en ligne. Cette banque permet quatre features :
- créer un compte bancaire
- effectuer un dépôt
- effectuer un retrait
- consulter l'état de son compte

Ce projet est une application web, exposant une API REST, construit avec une architecture hexagonale.

Le projet est divisé en trois modules :
- un module application, qui expose l'API REST
- un module domain, qui contient la logique métier et les use cases
- un module infrastructure, qui contient la base de données.

## Installation de Rust

## Mise en place du domaine

### Objectifs

- Création de structure et d'énumérations
- Création de fonction
- Création de méthode

### Enoncé

#### Domain 1

Création de la structure `BankAccount` contenant les champs suivants :
- `account_number` de type `String`
- `initial_amount` de type `u64`

Créer une fonction appelée `create_new_account` qui prend en paramètre un `account_number` et un `initial_amount` et qui retourne une instance de `BankAccount`.

Créer une méthode nommée `balance` qui retourne le solde du compte (pour le moment le montant initial).

#### Test

```cargo test --features domain1```

#### Domain 2

Création d'un enum `Transaction` contenant les variants suivants :
- `Deposit`
- `Withdraw`
-
avec chacun les champs suivants :
- `date` de type `DateTime<Utc>`
- `amount` de type `u64`

Création d'une méthode `amount` renvoyant le montant de la transaction.

Ajouter un champ `transactions` de type `Vec<Transaction>` à la structure `BankAccount`.

Mettre à jour la fonction `create_new_account` pour initialiser le champ `transactions` avec un vecteur vide.

#### Test

```cargo test --features domain2```

#### Domain 3

Création de deux méthodes `deposit` et `withdraw` sur la structure `BankAccount` prenant en paramètre un `amount` de type `u64`.
Ces méthodes créent une nouvelle instance de `Transaction` et l'ajoutent au champ `transactions`.

Mettre à jour la méthode `balance` pour prendre en compte les transactions.

#### Test

```cargo test --features domain3```

#### Domain 4

La structure UseCase est le service qui nous permettra d'interagir avec notre port et nos classe métier pour effectuer des actions tell que:
- Creation de nouveau compte
- Chargement d'un compte
- Retrait/Dépot d'argent

Dans cette étape le but est d'implémenter la méthode `create` qui permet de créer un nouveau compte à partir d'un montant initial et d'un numéro de compte

#### Test

```cargo test --features domain4```

#### Domain 5

Dans cette étape le but est d'implémenter la méthode `load` qui permet de charger un compte à partir d'un numéro de compte

#### Test

```cargo test --features domain5```

#### Domain 6

Dans cette étape le but est d'implémenter la méthode `deposit` qui permet de déposer un montant sur un compte

#### Test

```cargo test --features domain6```

#### Domain 7

Dans cette étape le but est d'implémenter la méthode `withdraw` qui permet de retirer un montant depuis un compte

#### Test

```cargo test --features domain7```

## Mise en place du repository

### Objectifs

- Implémentation d'un trait
- Manipulation de l'api collection
- Visibilité des élements du modules
- Trait PartialEq pour tester l'égalité

### Enoncé

#### Infra 1

Dans le module repository, créer une structure `BankAccountAdapter` contenant une `HashMap` :
- `accounts` : pour stocker les comptes

Création d'une implémentation de l'interface `BankAccountRepository` pour cette structure et implementer  `save_account` : stocker les information des comptes bancaires
Pour la méthode:
- `save_account` : stocker les informations des comptes bancaires

#### Test

```cargo test --features infra1```

### Infra 2 

Implementer la méthode :
- `load` : pour lire les informations d'un compte bancaire

#### Test

```cargo test --features infra2```

## Mise en place de la partie web

### Objectifs

- Implémentation d'un route avec le framework AXUM
- Serialisation et déserialisation des objets JSON
- Gestion des Mutex

### Enoncé

#### Application 1

Implementation de la route `create` permettant de créer un nouveau compte bancaire

```
POST /accounts 
{
    "initial_amount": 200,
    "account_id": "A001"
}

HTTP Response code 201 CREATED
```
#### Test

```cargo test --features application1```

#### Application 2

Implementation de la route `load` permettant de charger un élement depuis à l'aide de notre UseCase

```
GET /accounts/{acount_id}
HTTP Response code 201 CREATED
Response body:
{
    "initial_amount": 200,
    "account_id": "A001"
}

```
#### Test

```cargo test --features application2```

#### Application 3

Implementation de la route `deposit` permettant de déposer un montant sur un compte bancaire à l'aide de notre UseCase

```
POST /accounts/{acount_id}/deposits
{
    "amount": 200,
}
HTTP Response code 200 OK
```
#### Test

```cargo test --features application3```

#### Application 4

Implementation de la route `withdraw` permettant de retirer un montant depuis un compte bancaire à l'aide de notre UseCase

```
POST /accounts/{acount_id}/withdraws
{
    "amount": 200,
}
HTTP Response code 200 OK
```
#### Test

```cargo test --features application4```
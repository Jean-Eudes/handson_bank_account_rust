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
cargo test --features domain1

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
cargo test --features domain2

#### Domain 3

Création de deux méthodes `deposit` et `withdraw` sur la structure `BankAccount` prenant en paramètre un `amount` de type `u64`.
Ces méthodes créent une nouvelle instance de `Transaction` et l'ajoutent au champ `transactions`.

Mettre à jour la méthode `balance` pour prendre en compte les transactions.

#### Test
cargo test --features domain3

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

Création d'une implémentation de l'interface `BankAccountRepository` pour cette structure.

Pour les méthode:
- `save_account` : stocker les information des comptes bancaires
- `load` : lire les informations des comptes boncaires
  
## Mise en place de la partie web

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

Pour installer Rust, il faut suivre les instructions de la page https://rust-lang.org/tools/install/. L'installation diffère selon la plateforme (windows, linux ou macos).

### Installation sur windows

Pour windows, il y a deux façons d'installer rust :
- directement sous windows, mais avec en prérequis `Visual Studio C++ build`.
- en utilisant le sous système `WSL` (Windows Subsystem for Linux).

La seconde méthode est généralement plus simple.

### Installation sur linux ou macos

Pour Linux ou macos, si rustup est présent dans votre package manager, vous pouvez aussi l'installer avec si vous préférez.

Par exemple pour archlinux
```bash
sudo pacman -Sy rustup
rustup default stable # permet d'installer le compilateur rustc ainsi que cargo
```

Cet installeur installe aussi le compilateur Rust `rustc` et le gestionnaire de build `cargo`. 
Il permet par la suite de mettre à jour ces outils en utilisant la commande `rustup update`.

## Mise en place du domaine

Dans cette section, nous allons créer la structure de données représentant un compte bancaire, ainsi que les opérations possibles sur ce compte.

Notre domain contiendra uniquement la logique métier, sans dépendance vers un framework web ou une base de données, et va être inspiré du DDD.

Pour cette étape, l'ensemble du code est dans le module `domain`. Les tests sont codés, et vous pourrez passer à l'étape suivante une fois que les tests seront OK pour une étape.
La commande pour lancer les tests pour une étape est donnée en dessous de l'énoncé de chaque étape.

Des liens vers la documentation vous seront également fourni pour vous aider dans les exercices.

### Objectifs

- Création de structure et d'énumérations
- Création de fonctions
- Création de méthodes

### Étape 1

#### Énoncé

Création de la structure `BankAccount` contenant les champs suivants :
- `account_number` de type `String`
- `initial_amount` de type `u64`

Créer une fonction appelée `create_new_account` qui prend en paramètre un `account_number` et un `initial_amount` et qui retourne une instance de `BankAccount`.

Créer une méthode nommée `balance` qui retourne le solde du compte (pour le moment le montant initial).

#### Test
```bash
cargo test --features domain1
```

#### Lien utile

- https://doc.rust-lang.org/book/ch05-01-defining-structs.html

### Étape 2

#### Énoncé

Création d'un enum `Transaction` contenant les variants suivants :
- `Deposit`
- `Withdraw`

Avec pour chacun les deux champs suivants :
- `date` de type `DateTime<Utc>`
- `amount` de type `u64`

Création d'une méthode `amount` renvoyant le montant de la transaction.

Ajouter un champ `transactions` de type `Vec<Transaction>` à la structure `BankAccount`.

Mettre à jour la fonction `create_new_account` pour initialiser le champ `transactions` avec un vecteur vide.

#### Test
```bash
cargo test --features domain2
```

#### Lien utile
- https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
- https://doc.rust-lang.org/rust-by-example/std/vec.html

### Étape 3

#### Énoncé

Création de deux méthodes `deposit` et `withdraw` sur la structure `BankAccount` prenant en paramètre un `amount` de type `u64`.
Ces méthodes créent une nouvelle instance de `Transaction` et l'ajoutent au champ `transactions`.

Mettre à jour la méthode `balance` pour prendre en compte les transactions.

#### Test
```bash
cargo test --features domain3
```

#### Lien utile
- https://doc.rust-lang.org/std/vec/struct.Vec.html


### Étape 4

#### Énoncé

Dans le fichier `use_cases.rs`, créer deux fonctions :
- `deposit` : qui prend en paramètre un mutable reference vers un `BankAccount` et un `amount` de type `u64`, et qui appelle la méthode `deposit` sur le compte bancaire, et sauvegarde le compte dans le repository.
- `withdraw` : qui prend en paramètre un mutable reference vers un `BankAccount` et un `amount` de type `u64`, et qui appelle la méthode `withdraw` sur le compte bancaire et sauvegarde le compte dans le repository.
- Creation de nouveau compte
- Chargement d'un compte

#### Test
```bash
cargo test --features domain4
```

## Mise en place du repository

### Objectifs

- Implémentation d'un trait
- Manipulation de l'api collection
- Visibilité des élements du modules
- Trait PartialEq pour tester l'égalité

### Énoncé

#### Infra 1

Dans le module repository, créer une structure `BankAccountAdapter` contenant une `HashMap` :
- `accounts` : pour stocker les comptes

Création d'une implémentation de l'interface `BankAccountRepository` pour cette structure.

Pour les méthodes :
- `save_account` : stocker les informations des comptes bancaires
- `load` : lire les informations des comptes bancaires

```bash
cargo test --features infra1
```

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
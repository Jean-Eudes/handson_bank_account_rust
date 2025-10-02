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
- Utiliser un pointeur intelligent

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

#### Introduction

Dans cette partie, nous allons implémenter les use cases. Le role du use case est d'exposer des `services métiers` qui pourront ensuite être utilisé par notre API.
Dans notre cas, les use cases vont faire le lien entre nos objets du domaine et notre repository.

Pour celà, nous allons devoir `injecter` dans notre use case le repository qui va gérer la persistence.

Il y a deux concepts importants à comprendre en rust pour celà. Le premier concept est la notion d'allocation dynamique de mémoire.
Quand nous ne connaissons pas à l'avance la taille en mémoire de l'implementation de notre interface (c'est à dire au moment de la compilation), 
nous allons devoir utiliser un `pointeur intelligent` pour stocker cette implémentation.
En rust, c'est le type `Box<dyn Trait>` qui permet de faire celà. Le mot clef `dyn` indique que l'implémentation sera connue au moment de l'exécution, et le type `Box` indique que l'objet sera stocké sur le tas, 
car sa taille sera connu au moment de l'exécution.

#### Énoncé

Dans le fichier `use_cases.rs`, implementer les 4 fonctions suivantes :
- `create_bank_account` : qui permet de créer un compte bancaire ;
- `get_bank_account` : qui renvoie les information sur le compte bancaire ;
- `deposit` : permet de faire un dépôt sur le compte ;
- `withdraw` : permet de faire un retrait sur le compte.

Pour implémenter ces 4 méthodes, il va falloir charger un compte bancaire à partir du repository, effectuer une action métier en fonction de la méthode, et sauvegarder ensuite le résultat. 

#### Test
```bash
cargo test --features domain4
```

#### Lien utile

- https://doc.rust-lang.org/book/ch15-01-box.html
- https://doc.rust-lang.org/rust-by-example/trait/dyn.html
- https://doc.rust-lang.org/std/keyword.dyn.html


## Mise en place du repository

### Objectifs

- Implémentation d'un trait
- Manipulation de l'api collection
- Visibilité des élements du modules
- Trait PartialEq pour tester l'égalité

### Étape 4

#### Introduction

L'objectif de cette étape est d'implémenter une base de données en mémoire, en utilisant une `HashMap` pour stocker les comptes bancaires.

La complexité de cette étape va être lié au système de type de rust. En rust, dans un environnement multi threading comme un serveur web,
Il n'est pas possible que deux threads accèdent en même temps à une même donnée. Pour celà, le compilateur nous oblige à utiliser un `Mutex` (mutual exclusion) qui va permettre de protéger l'accès à une donnée.

Le second soucis va être lié à se que l'on appelle le système `d'ownership` de rust. En rust, chaque variable ne peut avoir qu'un seul propriétaire. 
Dans notre cas, c'est notre HashMap qui va être propriétaire des comptes bancaires, nous allons être obligé de renvoyer une copie de l'objet via la méthode `clone`. 

#### Énoncé

Dans le module repository, Nous avons une structure `BankAccountAdapter` qui contient déjà une `HashMap` nous permettant de stocker les comptes.

Création d'une implémentation de l'interface `BankAccountRepository` pour cette structure.

Implémenter les méthodes :
- `save_account` : stocker les informations des comptes bancaires
- `load` : lire les informations des comptes bancaires

#### Test

```bash
cargo test --features infra1
```

#### Tips

Pour le `Mutex`, nous remarquons que le unlock va automatiquement être appelé quand nous sortons de la méthode.
Ce pattern s'appelle le RAII (Resource Acquisition Is Initialization).
Quand une variable sort de son scope, sa méthode `drop` est automatiquement appelée, ce qui permet de libérer les ressources, donc dans notre cas de libérer le lock.

#### Lien utile
- https://doc.rust-lang.org/std/collections/struct.HashMap.html
- https://doc.rust-lang.org/rust-by-example/std/hash.html
- https://doc.rust-lang.org/std/sync/struct.Mutex.html
- https://doc.rust-lang.org/rust-by-example/scope/raii.html


## Mise en place de la partie web

#### Introduction

Dans cette partie, nous allons implémenter la partie REST de notre micro services.
Pour celà, nous avons choisi le framework axum, qui possède une syntaxe à la `express` (framework JS)
pour exposer nos routes.

Aum est un framework qui fair partie de l'écosystème d'un autre framework très populaire : `tokio`.
`tokio` est un framework permettant de faire de la programmation asynchrone en rust, à l'aide de la syntaxe `async/await`. (très proche de la syntaxe JS)

### Objectifs

- Implémentation d'une route avec le framework AXUM
- Serialisation et déserialisation des objets JSON

### Étape 5

#### Énoncé

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

#### Lien utile
- https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
- https://docs.rs/axum/latest/axum/
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
- en utilisant le sous-système `WSL` (Windows Subsystem for Linux).

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

Notre domaine contiendra uniquement la logique métier, sans dépendance vers un framework web ou une base de données, et va être inspiré du DDD.

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
- `initial_amount` de type `i64`

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
- `amount` de type `i64`

Création d'une méthode `amount` renvoyant le montant de la transaction, qui renvoie le montant en valeur relative.
C'est à dire que pour un dépôt, le montant sera positif, et pour un retrait le montant doit être négatif.

Ajouter un champ `transactions` de type `Vec<Transaction>` à la structure `BankAccount`.

Mettre à jour la fonction `create_new_account` pour initialiser le champ `transactions` avec un vecteur vide.

#### Test
```bash
cargo test --features domain2
```

#### Lien utile
- https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
- https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html
- https://doc.rust-lang.org/rust-by-example/std/vec.html

### Étape 3

#### Énoncé

Création de deux méthodes `deposit` et `withdraw` sur la structure `BankAccount` prenant en paramètre un `amount` de type `i64`.
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

Dans cette partie, nous allons implémenter les use cases. Le rôle du use case est d'exposer des `services métiers` qui pourront ensuite être utilisés par notre API.
Dans notre cas, les use cases vont faire le lien entre nos objets du domaine et notre repository.

Pour cela, nous allons devoir `injecter` dans notre use case le repository qui va gérer la persistance.

Il y a deux concepts importants à comprendre en rust pour cela. Le premier concept est la notion d'allocation dynamique de mémoire.
Quand nous ne connaissons pas à l'avance la taille en mémoire de l'implémentation de notre interface (c'est-à-dire au moment de la compilation), 
nous allons devoir utiliser un `pointeur intelligent` pour stocker cette implémentation.
En rust, c'est le type `Box<dyn Trait>` qui permet de faire cela. Le mot clef `dyn` indique que l'implémentation sera connue au moment de l'exécution. 

Le type `Box` indique que l'objet sera stocké sur le tas, car sa taille sera connue au moment de l'exécution.

#### Énoncé

Dans le fichier `use_case.rs`, implémenter les 4 fonctions suivantes :
- `create` : qui permet de créer un compte bancaire ;
- `fetch` : qui renvoie les informations sur le compte bancaire ;
- `deposit` : permet de faire un dépôt sur le compte ;
- `withdraw` : permet de faire un retrait sur le compte.

Pour implémenter ces 4 méthodes, il va falloir charger un compte bancaire à partir du repository, effectuer une action métier en fonction de la méthode, et sauvegarder ensuite le résultat. 

#### Test
```bash
cargo test --features domain4
```

### Étape 4 bis (aide à la compréhension)

Si nous regardons la struct `BankAccount`, nous remarquons qu'une syntaxe `#[derive()]` est déjà présente. Cette syntaxe correspond à une macro qui
nous génère du code, en fonction des attributs de la macro.

Dans notre cas précis, cette macro va implémenter les trois traits (interfaces) sus nommés, et générer le code correspondant.

Une macro rust va générer du code au moment du build de notre projet. Ce code peut être visible avec la commande `cargo expand`.

``` rust
#[automatically_derived]
impl ::core::fmt::Debug for BankAccount {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "BankAccount")
    }
}
```

#### Lien utile

- https://doc.rust-lang.org/book/ch15-01-box.html
- https://doc.rust-lang.org/rust-by-example/trait/dyn.html
- https://doc.rust-lang.org/std/keyword.dyn.html


## Mise en place du repository

### Objectifs

- Implémentation d'un trait
- Manipulation de l'api collection
- Type Option

### Étape 5

#### Introduction

L'objectif de cette étape est d'implémenter une base de données en mémoire, en utilisant une `HashMap` pour stocker les comptes bancaires.

La complexité de cette étape va être liée au système de type de rust. En rust, dans un environnement multi-threading comme un serveur web,
il n'est pas possible que deux threads accèdent en même temps à une même donnée. Pour cela, le compilateur nous oblige à utiliser un `Mutex` (mutual exclusion) qui va permettre de protéger l'accès à une donnée.

Le second souci va être lié à ce que l'on appelle le système `d'ownership` de rust. En rust, chaque variable ne peut avoir qu'un seul propriétaire. 
Dans notre cas, c'est notre HashMap qui va être propriétaire des comptes bancaires, nous allons être obligé de renvoyer une copie de l'objet via la méthode `clone`. 

#### Énoncé

Dans le module repository, nous avons une structure `BankAccountAdapter` qui contient déjà une `HashMap` nous permettant de stocker les comptes.

Création d'une implémentation du trait `BankAccountPort` pour cette structure.

Implémenter les méthodes :
- `save_account` : stocker les informations des comptes bancaires
- `load` : lire les informations des comptes bancaires. Cette méthode renvoie une `Option`, qui doit être vide si le compte bancaire n'est pas présent.
Il va aussi être nécessaire de faire un clone de l'objet BankAccount. (un objet rust ne peut avoir qu'un seul propriétaire).

#### Test

```bash
cargo test --features infra1
```

#### Tips

La méthode `lock` du Mutex renvoie un type `Result` que nous verrons par la suite. Pour le moment, vous pouvez utiliser la méthode `unwrap` 
qui permet de récupérer la valeur contenue dans le `Result`, ou de faire planter le programme en cas d'erreur. (C'est globalement une très mauvaise pratique, mais cela permet de faciliter l'exercice).

Une Option est un type énuméré qui permet de représenter la présence ou l'absence d'une valeur. Il a deux variantes : Some et None.
Comme le `null` n'existe pas en rust, c'est la seule façon en rust de représenter l'absence d'une valeur.

Pour le `Mutex`, nous remarquons que le unlock va automatiquement être appelé quand nous sortons de la méthode.
Ce pattern s'appelle le RAII (Resource Acquisition Is Initialization).
Quand une variable sort de son scope, sa méthode `drop` est automatiquement appelée, ce qui permet de libérer les ressources, donc dans notre cas de libérer le lock.

#### Lien utile
- https://doc.rust-lang.org/std/collections/struct.HashMap.html
- https://doc.rust-lang.org/rust-by-example/std/hash.html
- https://doc.rust-lang.org/std/sync/struct.Mutex.html
- https://doc.rust-lang.org/rust-by-example/scope/raii.html
- https://doc.rust-lang.org/std/option/enum.Option.html


## Mise en place de la partie web

#### Introduction

Dans cette partie, nous allons implémenter la partie REST de notre microservice.
Pour cela, nous avons choisi le framework axum, qui possède une syntaxe à la `express` (framework JS)
pour exposer nos routes.

Axum fait lui même partie de l'écosystème d'un autre framework très populaire : `tokio`, qui permet de faire de la programmation asynchrone en rust, à l'aide de la syntaxe `async/await`. 
(très proche de la syntaxe JS)

### Objectifs

- Implémentation de route avec le framework AXUM
- Serialisation et déserialisation de `struct` rust en objet JSON
- Gestion des erreurs en rust.

### Étape 6

#### Énoncé

Dans cette partie, nous allons implémenter les 4 routes qui nous manquent pour finaliser notre microservice.
Si vous regardez le fichier `main.rs`, vous verrez que le serveur est déjà implémenté, et que les routes sont déjà préconfigurées.
Les implémentations des routes sont dans le module `resource`.

Pour commencer, nous allons implémenter la route `create` permettant de créer un nouveau compte bancaire

```
POST /accounts 
{
    "initial_amount": 200,
    "account_id": "A001"
}

HTTP Response code 201 CREATED
```
Cette route doit retourner un code HTTP 201 en cas de succès.

#### Test

```bash
cargo test --features application1
```

#### Énoncé

Implémentation de la route `load` permettant de charger un élément depuis notre repository à l'aide de notre use case.

```
GET /accounts/{account_id}
HTTP Response code 201 CREATED
Response body:
{
    "initial_amount": 200,
    "account_id": "A001"
}
```
Si l'élément n'existe pas, renvoyer un code HTTP 404.

#### Tips

Pour gérer les erreurs, nous allons utiliser le type `Result` de rust. Ce type permet de renvoyer un résultat en cas de succès, et un autre résultat en cas d'erreur.
Il n'existe pas vraiment de notion d'exception.

#### Test

```bash
cargo test --features application2
```

#### Énoncé

Implémentation de la route `deposit` permettant de déposer un montant sur un compte bancaire à l'aide de notre UseCase

```
POST /accounts/{account_id}/deposits
{
    "amount": 200,
}
HTTP Response code 200 OK
```
#### Test

```bash
cargo test --features application3
```

#### Énoncé

Implémentation de la route `withdraw` permettant de retirer un montant depuis un compte bancaire à l'aide de notre UseCase

```
POST /accounts/{account_id}/withdraws
{
    "amount": 200,
}
HTTP Response code 200 OK
```
#### Test

```bash
cargo test --features application4
```

#### Tips

La sérialisation et la désérialisation des objets JSON est faite automatiquement par la librairie `serde`.

Vous pouvez lancer votre serveur web à l'aide de la commande suivante :
```bash
cargo run
```
Cette commande lance le serveur web sur le port 3000. Vous pouvez ensuite le tester en utilisant par exemple une commande `curl`, ou tout autre client HTTP

Exemple de commande curl :
```bash
curl http://localhost:3000/accounts \
  -H "Content-Type: application/json" \
  -d '{"initial_amount": 200, "account_id": "A002"}'
curl http://localhost:3000/accounts/A002/deposits \
  -H "Content-Type: application/json" \
  -d '{"amount": 50}'
curl http://localhost:3000/accounts/A002/withdraws \
  -H "Content-Type: application/json" \
  -d '{"amount": 100}'
curl http://localhost:3000/accounts/A002
```
#### Lien utile
- https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
- https://tokio.rs/tokio/tutorial
- https://docs.rs/axum/latest/axum/
- https://doc.rust-lang.org/std/result/
- https://doc.rust-lang.org/rust-by-example/error/result.html

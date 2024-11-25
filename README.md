## Introduction

Implémentation d'un **bump allocator** en `no_std`, une méthode de gestion de la mémoire qui fonctionne en allouant de la mémoire de manière à partir de la heap.


## Compilation et vérification

`- RUSTFLAGS="-C link-arg=-nostartfiles" cargo build`

Ensuite, pour vérifier le fonctionnement, on utilise GDB :
`- gdb target/debug/rust`

On peut déjà vérifier que la heap s'initialise bien.
Lorsqu'on lance gdb, on va utiliser `b _ start` suivis de `run`et finir par un `ctrl C` lorsque nous seront dans une loop

à partir de la, ont peut vérifier avec `info proc mappings`que la heap est bien initialisé dans "objfile"

On peut ensuite vérifie si l'allocation s'est effectué avec `ìnfo registers`, on regarde le resultat dans RAX (ici 0x55555555b000)
on récupère cette valeur pour regarder à l'interieur de ce registre, `x/16x 0x55555555b000`, on peut voir que l'allocation s'est bien effectué

## Pourquoi BumpAllocator

J'ai choisi d'implémenter un **bump allocator** car son implémentation en `no_std` est plus simple comparée à d'autres types d'allocateurs, avec ce type de gestion de mémoire je n'ai à gérer que l'allocation, étant donné qu'il ne prend pas en charge les désallocations. ce qui simplifie beaucoup la logique.

Le fonctionnement du **bump allocator** est que à chaque demande d'allocation, il avance un pointeur qui indique la fin de la mémoire déjà allouée. Ce qui évite les opération plus difficile qui consite a la recherche de blocs libres.

C'est surtout grâce a cette simplicité que le **bump allocator** est considéré comme le type d'allocateur le plus rapide.

Mais cette rapidité est du fait qu'il est impossible de libérer ou de réutiliser la mémoire une fois allouée. Ce qui fait que ce type d'allocateur est adapté aux scénarios où les allocations sont effectuées dans un ordre linéaire et où toute la mémoire est libérée en une seule fois.
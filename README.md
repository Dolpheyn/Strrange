# Strrange

Strrange (Stall Arrangement) - A Permutation Genetic Algorithm Use Case

## Problem Statement

Stalls in [bazaar][Bazaar] ramadhan can be categorized into several categories.
It is often best to not have stalls in the same category to be close to each
other to maximise the distribution of options along the bazaar. For example, if
all stalls that sell drinks are clumped to each other at one end of the bazaar,
then people who had walked past that part of the bazaar would have to walk a
long way back after failing to find another option on the other end. This would
stress a lot—especially fasting—of people, as they would have to walk a long
distance just to get that *air katira*.

This problem can be categorized as a permutation problem with an objective
function of minimizing the adjacent stalls with the same category. The phenotype
space P is the set of all possible stall configurations. The quality q(p) of any
phenotype p ∈ P can be quantified by the number of adjacent stalls with the same
category. The lower q(p) is, the better the phenotype. A zero value, q(p) = 0,
indicates the perfect solution, whenever possible. From this observation we can
conclude that the objective function is to be minimised, and have an optimal
value 0 whenever possible.

And of course, its a made-up problem I created for a uni assignment.

## Representation

Every chromosome will have the permutation encoding of all the stalls in a
particular bazaar ramadhan, which represents numbers in a sequence(a string of
numbers).

## Objective Function q(p)

Number of adjacent stalls with the same category. Goal: minimise.

## Termination Condition

When q(p) = 0, or reach the number of maximum iteration.

## Selection Mechanism

Rank-based selection.

## Combination Mechanism

One-point crossover on 2 best parents with 100% chance to happen.

## Mutation Mechanism

Swap with 30% chance to happen.

[Bazaar]: [https://en.wikipedia.org/wiki/Bazaar]

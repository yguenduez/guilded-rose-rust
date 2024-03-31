# Gilded Rose Refactoring Kata

Originally taken
from: [https://github.com/emilybache/GildedRose-Refactoring-Kata](https://github.com/emilybache/GildedRose-Refactoring-Kata).
I also used the book [Refactoring](https://martinfowler.com/books/refactoring.html) as reference.

**Summing it up**: Refactoring code - so it's maintainable again. The starting code sample is exemplary for legacy code.
A
lot of nested branches, duplicated code, complicated logic and no tests.

## Requirements

For the requirements of this challenge, have a look [here](GildedRoseRequirements.txt)

## Steps taken

Here are the steps I took. I tried to make them as small as possible - see the git history for detailed reference.

### V0.0.1

- Starting point - clone repo and throw anything out except the rust part

### V0.0.2

- Writing the test suite in BDD Style ([given-when-then](https://en.wikipedia.org/wiki/Given-When-Then)).
- Change Sulfaras and Backstage passes to the Strings defined in the requirements

### V0.0.3

- Start with the most intended (most nested) parts of the code
- Change Conditions
- Inverse Conditions
- Make else the "default" branch for normal items
- Remove Conditions by min or max clipping the quality value

### V0.0.4

- Merge branches, so the conditions, e.g. `Backstage passes` is not checked twice

### V0.0.5

- Separate immutable values from modifications. Rather Compute the quality value/increment, and assign the computed
  value in the end: Calculate the quality increment
- Move assignments out the branches to top level
- Move queries to top level
- Extract methods on calculations in between branch-bodies (if-else)

### V0.0.6

- Also calculate the sell-in value: Out of symmetry to the quality value
- Refactor into static function

### V0.0.7

- Instead of the increment, calculate quality directly
- Remove top level Condition for `Sulfaras`. Different abstraction level
    - By moving the clipping (min/max) to the new `calculate_quality` method
- For symmetry, also calculate sell_directly instead of its increment

### V0.0.8

- Instead of `if-else` branching, use polymorphism: Therefore move logic into classes

### V0.0.9

- Do the same with the `sell_in` calculation. But use default behaviour on a trait, as only `Sulfaras` differs.

### V0.0.9 (V0.0.10)

- Move increment methods from gilded to the new classes, because of different level ob abstraction
- Move shared increment logic to its own class, so the `GildedRose` class is free of the "how to increment" logic

### V0.0.9 (V0.0.11)

The refactoring is fine for me, of course you can still refactor. But now adding the `Conjured` items seems to be
  pretty straight forward  
What do I need to implement for it?:
- Factory returning a new class
- Implement both needed traits: `CalculateQuality` and `CalculateSellIn`
- Calculate the increment as delegate: But we can re-use everything.

Done:
- Write test suite for conjured cookie
- Implement as stated above

# Learnings

- Write tests first
- Even if the code exists already
- Make the tests fail first, therefore temporarily change the production code, so you know the test actually tests the
  part of the code you actually want to test :)
- Make **small** refactoring steps, then test. If the tests fail, undo your changes. No need for debugging.
- Change things when needed. If a prior refactoring does not fit anymore - change it to your current needs (e.g. a name
  for function does not fit anymore after two commits, change it)

# Note 

- Tags v0.0.10 and v.0.0.11 do not exist anymore, as they got amended into the v0.0.9

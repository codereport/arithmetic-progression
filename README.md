This repo contains solutions to the Arithmetic Progression problem. It was coverd in this YouTube video (coming). It can be found:

* LeetCode: [Problem 1502](https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/description/)
* Perl Weekly Challenge: [Week 351, Problem 2](https://theweeklychallenge.org/blog/perl-weekly-challenge-351/)

It can be solved trivially if you have three algorithms:

* sort
* deltas or [map_adjacent](https://hoogletranslate.com/?q=5&type=by-algo-id)
* [all_equal](https://hoogletranslate.com/?q=51&type=by-algo-id)

### Array Language Solutions

|  Language  |       Solution       |
| :--------: | :------------------: |
|   Jelly    |        `ṢIE`         |
|    Uiua    |      `/↧⧈=⧈-⍆`       |
|    Uiua    |      `=1⧻◴⧈-⍆`       |
|    Kap     |      `1=≢∪2-/∧`      |
|    Kap     |     `∧/2=/2-/∧`      |
| Dyalog APL |   `∧/2=/2-/⊂⍤⍋⍛⌷`    |
|    BQN     |   `1=·≠∘⍷·-´˘2↕∧`    |
| Dyalog APL |  `1=(≢∘∪2-/⊂⍤⍋⍛⌷)`   |
|    BQN     |  `∧´·=´˘2↕·-´˘2↕∧`   |
|   Jello    | `sort deltas all_eq` |

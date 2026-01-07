import Data.List (sort)
import Data.List.HT (mapAdjacent, allEqual)

arithmeticProgression = allEqual
                      . mapAdjacent (-)
                      . sort

main = print $ arithmeticProgression [3,1,7,9,5]

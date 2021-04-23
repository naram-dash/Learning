doubleMe x = x + x

-- doubleMe 9
-- doubleMe 8.3

-- doubleUs x y = x * 2 + y * 2
doubleUs x y = doubleMe x + doubleMe y

-- doubleUs 4 9
-- doubleUs 2.3 34.2
-- doubleUs 28 88 + doubleMe 123

doubleSmallNumber x = if x > 100 then x else x * 2

doubleSmallNumber' x = (if x > 100 then x else x * 2) + 1

conanO'Brien = "It's a-me, Conan O'Brien!"
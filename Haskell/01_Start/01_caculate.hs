-- ghci

-- Number
2 + 15
49 * 100
1892 - 1472
5 / 2
(50 * 100) - 4999
50 * 100 - 4999
50 * (100 - 4999)
5 * (-3) -- be aware of minus

-- Boolean
True && False
True && True
False || True
not False
not (True && True)

-- Boolean Operator
5 == 5
1 == 0
5 /= 5
5 /= 4
"hello" == "hello"

-- function

succ 8
min 9 10
min 3.4 3.2
max 100 101
succ 9 + max 5 4 + 1 -- function application has the fastest priority
(succ 9) + max (5 4) + 1
succ 9 * 10 -- is 100
succ (9 * 10) -- is 91

div 92 10
92 `div` 10 -- infix function calling
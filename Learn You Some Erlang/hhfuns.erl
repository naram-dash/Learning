-module(hhfuns).

-compile(export_all).

one() ->
  1.

two() ->
  2.

add(X, Y) ->
  X() + Y().

increment([]) ->
  [];
increment([H | T]) ->
  [H + 1 | increment(T)].

decrement([]) ->
  [];
decrement([H | T]) ->
  [H - 1 | decrement(T)].

map(_, []) ->
  [];
map(F, [H | T]) ->
  [F(H) | map(F, T)].

incr(X) ->
  X + 1.

decr(X) ->
  X - 1.

% base(A) ->
%   B = A + 1,
%   F = fun() -> A * B end,
%   F().
base(A) ->
  B = A + 1,
  fun() -> A * B end().

a() ->
  Secret = "pony",
  fun() -> Secret end.

b(F) ->
  "a/0's password is " ++ F().

even(L) ->
  lists:reverse(even(L, [])).

even([], Acc) ->
  Acc;
even([H | T], Acc) when H rem 2 == 0 ->
  even(T, [H | Acc]);
even([_ | T], Acc) ->
  even(T, Acc).

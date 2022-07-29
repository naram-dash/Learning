-module(dolphins).

-compile(export_all).

dolphin1() ->
  receive
    do_a_flip ->
      io:format("how about no? ~n");
    fish ->
      io:format("so long and thanks for all the fish! ~n");
    _ ->
      io:format("heh were smarter than you humans. ~n")
  end.
